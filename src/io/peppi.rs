pub mod de;
pub mod ser;

use serde::{Deserialize, Serialize};
use std::{
	fmt,
	io::{self, Error, Read},
	str,
};

use crate::{
	io::{parse_u8, PosError, TrackingReader},
	model::game::immutable::Game,
};

pub use ser::write;

/// Current version of the Peppi format
pub const CURRENT_VERSION: Version = Version(2, 0);

/// Peppi files are TAR archives, and are guaranteed to start with `peppi.json`
pub const FILE_SIGNATURE: [u8; 10] = [0x70, 0x65, 0x70, 0x70, 0x69, 0x2e, 0x6a, 0x73, 0x6f, 0x6e];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Version(pub u8, pub u8);

impl fmt::Display for Version {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}.{}", self.0, self.1)
	}
}

impl str::FromStr for Version {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut i = s.split('.');
		match (i.next(), i.next(), i.next()) {
			(Some(major), Some(minor), None) => Ok(Version(parse_u8(major)?, parse_u8(minor)?)),
			_ => Err(err!("invalid Peppi version: {}", s.to_string())),
		}
	}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Peppi {
	pub version: Version,
	pub slp_hash: String,
}

pub(crate) fn assert_current_version(version: Version) -> io::Result<()> {
	if version == CURRENT_VERSION {
		Ok(())
	} else {
		Err(err!(
			"unsupported version ({} != {})",
			version,
			CURRENT_VERSION
		))
	}
}

/// Parses a Peppi replay from `r`, returning a `Game` and a `Peppi`.
pub fn read<R: Read>(r: &mut R, opts: Option<&de::Opts>) -> Result<(Game, Peppi), PosError> {
	let mut r = TrackingReader { pos: 0, reader: r };
	de::read(&mut r, opts).map_err(|e| PosError {
		error: e,
		pos: r.pos,
	})
}