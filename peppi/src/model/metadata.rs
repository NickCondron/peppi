use std::{
	collections::HashMap,
	io::Result,
};

use chrono::{DateTime, Utc};
use log::warn;
use serde_json::{Map, Value};

use crate::model::{
	enums::character,
	game::FIRST_FRAME_INDEX,
	primitives::Port,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Platform {
	Dolphin,
	Network,
	Nintendont,
	Unknown(String),
}

impl From<Platform> for String {
	fn from(platform: Platform) -> String {
		match platform {
			Platform::Dolphin => String::from("dolphin"),
			Platform::Network => String::from("network"),
			Platform::Nintendont => String::from("nintendont"),
			Platform::Unknown(s) => s,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
	pub date: Option<DateTime<Utc>>,
	pub duration: Option<usize>,
	pub platform: Option<Platform>,
	pub console: Option<String>,
	pub players: Option<Vec<Player>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
	pub port: Port,
	pub characters: Option<HashMap<character::Internal, usize>>,
	pub netplay: Option<Netplay>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Netplay {
	pub code: String,
	pub name: String,
}

fn date(json: &Map<String, Value>) -> Result<Option<DateTime<Utc>>> {
	let date_too_short = "2000-01-01T00:00:00".parse::<DateTime<Utc>>();
	match json.get("startAt") {
		None => Ok(None),
		Some(Value::String(start_at)) => match start_at.parse::<DateTime<Utc>>() {
			Ok(start_at) => Ok(Some(start_at)),
			e if e == date_too_short =>
				format!("{}Z", start_at).parse::<DateTime<Utc>>()
					.map(Some)
					.map_err(|e| err!("metadata.startAt: parse error: {:?}, {:?}", e, start_at)),
			Err(e) => Err(err!("metadata.startAt: parse error: {:?}, {:?}", e, start_at)),
		},
		start_at => Err(err!("metadata.startAt: expected str, but got: {:?}", start_at)),
	}
}

fn duration(json: &Map<String, Value>) -> Result<Option<usize>> {
	match json.get("lastFrame") {
		None => Ok(None),
		Some(Value::Number(last_frame)) => match last_frame.as_i64() {
			Some(last_frame) => match usize::try_from(last_frame - FIRST_FRAME_INDEX as i64 + 1) {
				Ok(duration) => Ok(Some(duration)),
				Err(e) => Err(err!("metadata.lastFrame: value out of range: {:?}, {:?}", last_frame, e)),
			},
			None => Err(err!("metadata.lastFrame: expected i64, but got: {:?}", last_frame)),
		},
		last_frame => Err(err!("metadata.lastFrame: expected number, but got: {:?}", last_frame)),
	}
}

fn platform(json: &Map<String, Value>) -> Result<Option<Platform>> {
	match json.get("playedOn") {
		None => Ok(None),
		Some(Value::String(platform)) => match platform.as_str() {
			"dolphin" => Ok(Some(Platform::Dolphin)),
			"network" => Ok(Some(Platform::Network)),
			"nintendont" => Ok(Some(Platform::Nintendont)),
			s => {
				warn!("Unrecognized metadata.playedOn {:?}", s);
				Ok(Some(Platform::Unknown(platform.clone())))
			},
		},
		played_on => Err(err!("metadata.playedOn: expected str, but got: {:?}", played_on)),
	}
}

fn parse_characters(characters: &Map<String, Value>) -> Result<HashMap<character::Internal, usize>> {
	characters.iter().map(|(k, v)| {
		let k = k.parse::<u8>().map_err(|e| err!("metadata.players.N.characters: invalid character: {:?}, {:?}", k, e))?;
		match v {
			Value::Number(v) => match v.as_u64() {
				Some(v) => Ok((
					character::Internal(k),
					usize::try_from(v).map_err(|e| err!("metadata.players.N.characters.{}: invalid duration: {:?}, {:?}", k, v, e))?,
				)),
				None => Err(err!("metadata.players.N.characters.{}: expected u64, but got: {:?}", k, v)),
			},
			v => Err(err!("metadata.players.N.characters.{}: expected number, but got: {:?}", k, v)),
		}
	}).collect()
}

fn metadata_player(port: Port, player: &Map<String, Value>) -> Result<Player> {
	Ok(Player {
		port: port,
		characters: match player.get("characters") {
			Some(Value::Object(characters)) => match parse_characters(characters) {
				Ok(characters) => Some(characters),
				Err(e) => return Err(err!("metadata.players.N.characters: parse error: {:?}, {:?}", e, characters)),
			},
			characters => return Err(err!("metadata.players.N.characters: expected map, but got: {:?}", characters)),
		},
		netplay: match player.get("names") {
			None => None,
			Some(Value::Object(names)) => match names.get("code") {
				None => None,
				Some(Value::String(code)) => match names.get("netplay") {
					None => { warn!("Ignoring netplay name without code"); None },
					Some(Value::String(name)) => Some(Netplay {
						code: code.clone(),
						name: name.clone(),
					}),
					name => return Err(err!("metadata.players.N.names.netplay: expected str, but got: {:?}", name)),
				},
				code => return Err(err!("metadata.players.N.names.code: expected str, but got: {:?}", code)),
			},
			names => return Err(err!("metadata.players.N.names: expected map, but got: {:?}", names)),
		},
	})
}

fn players(json: &Map<String, Value>) -> Result<Option<Vec<Player>>> {
	match json.get("players") {
		None => Ok(None),
		Some(Value::Object(players)) => {
			let mut result = Vec::<Player>::new();
			let mut players: Vec<_> = players.iter().collect();
			players.sort_by_key(|(k, _)| k.parse::<usize>().unwrap_or(0));
			for (port, player) in players {
				match port.parse::<u8>() {
					Ok(port) => match Port::try_from(port) {
						Ok(port) => match player {
							Value::Object(player) => result.push(metadata_player(port, player)?),
							player => return Err(err!("metadata.players.{:?}: expected map, but got: {:?}", port, player)),
						},
						Err(e) => return Err(err!("metadata.players: invalid port: {}, {:?}", port, e)),
					},
					Err(e) => return Err(err!("metadata.players: invalid port: {}, {:?}", port, e)),
				};
			}
			match result.len() {
				0 => Ok(None),
				_ => Ok(Some(result)),
			}
		},
		players => Err(err!("metadata.players: expected map, but got: {:?}", players)),
	}
}

fn console(json: &Map<String, Value>) -> Result<Option<String>> {
	match json.get("consoleNick") {
		None => Ok(None),
		Some(Value::String(console_nick)) => Ok(Some(console_nick.clone())),
		console_nick => Err(err!("metadata.consoleNick: expected str, but got: {:?}", console_nick)),
	}
}

impl Metadata {
	pub fn parse(json: &Map<String, Value>) -> Result<Metadata> {
		Ok(Metadata {
			date: date(json)?,
			duration: duration(json)?,
			platform: platform(json)?,
			players: players(json)?,
			console: console(json)?,
		})
	}

	pub fn to_map(&self) -> Map<String, Value> {
		let mut map = Map::new();

		self.date.map(|date| {
			map.insert(String::from("startAt"), date.to_rfc3339().into());
		});
		self.duration.map(|duration| {
			let duration = i64::try_from(duration - 1).unwrap() + FIRST_FRAME_INDEX as i64;
			map.insert(String::from("lastFrame"), duration.into());
		});
		self.platform.clone().map(|platform| {
			map.insert(String::from("playedOn"), String::from(platform).into() );
		});
		self.console.clone().map(|console| {
			map.insert(String::from("consoleNick"), console.into());
		});
		self.players.as_ref().map(|players| {
			let mut players_map = Map::new();
			for p in players {
				let mut player_map = Map::new();

				if let Some(characters) = &p.characters {
					let characters_map = characters.iter().map(|(internal, frames)| {
						let internal = internal.0.to_string();
						let frames = (*frames).into();
						(internal, frames)
					}).collect::<Map<String, Value>>();
					player_map.insert(String::from("characters"), characters_map.into());
				}

				if let Some(netplay) = &p.netplay {
					let mut names_map = Map::new();
					names_map.insert(String::from("netplay"), netplay.name.clone().into());
					names_map.insert(String::from("code"), netplay.code.clone().into());
					player_map.insert(String::from("names"), names_map.into());
				}

				let port_string = match p.port {
					Port::P1 => String::from("0"),
					Port::P2 => String::from("1"),
					Port::P3 => String::from("2"),
					Port::P4 => String::from("3"),
				};
				players_map.insert(port_string, player_map.into());
			}
			map.insert(String::from("players"), players_map.into());
		});

		map
	}
}
