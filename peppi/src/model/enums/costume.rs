use std::fmt;
use super::character::External;

macro_rules! costume {
	($name: ident {
		$unknown: ident,
		$( $variant: ident ( $variant_type: ident ) ( $red: expr, $green: expr, $blue: expr )  => $external: ident ),* $(,)?
	}) => {
		#[derive(Copy, Clone, PartialEq, Eq, serde::Serialize)]
		#[serde(untagged)]
		pub enum $name {
			$unknown(u8),
			$( $variant($variant_type), )*
		}

		impl $name {
			pub fn from(value: u8, character: External) -> $name {
				match character {
					$( External::$external => $name::$variant($variant_type(value)), )*
					_ => $name::$unknown(value),
				}
			}

			pub fn default(character: External) -> $name {
				match character {
					$( External::$external => $name::$variant($variant_type(0)), )*
					_ => $name::$unknown(0),
				}
			}

			pub fn red(character: External) -> $name {
				match character {
					$( External::$external => $name::$variant($variant_type($red)), )*
					_ => $name::$unknown(0),
				}
			}

			pub fn green(character: External) -> $name {
				match character {
					$( External::$external => $name::$variant($variant_type($green)), )*
					_ => $name::$unknown(0),
				}
			}

			pub fn blue(character: External) -> $name {
				match character {
					$( External::$external => $name::$variant($variant_type($blue)), )*
					_ => $name::$unknown(0),
				}
			}

			pub fn character(self) -> Option<External> {
				match self {
					$name::$unknown(_) => None,
					$( $name::$variant(_) => Some(External::$external), )*
				}
			}
		}

		impl From<$name> for u8 {
			fn from(state: $name) -> u8 {
				match state {
					$name::$unknown(s) => s,
					$( $name::$variant(s) => s.0, )*
				}
			}
		}

		impl fmt::Debug for $name {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				 match *self {
					$name::$unknown(s) => write!(f, "{:?}", s),
					$( $name::$variant(s) => write!(f, "{:?}", s), )*
				}
			}
		}
	}
}

costume!(Costume {
	Unknown,
	CaptainFalcon(CaptainFalcon) (2, 4, 5) => CAPTAIN_FALCON,
	DonkeyKong(DonkeyKong) (2, 3, 4) => DONKEY_KONG,
	Fox(Fox) (1, 3, 2) => FOX,
	GameAndWatch(GameAndWatch) (1, 3, 2) => GAME_AND_WATCH,
	Kirby(Kirby) (3, 4, 2) => KIRBY,
	Bowser(Bowser) (1, 0, 2) => BOWSER,
	Link(Link) (1, 0, 2) => LINK,
	Luigi(Luigi) (3, 0, 2) => LUIGI,
	Mario(Mario) (0, 4, 3) => MARIO,
	Marth(Marth) (1, 2, 0) => MARTH,
	Mewtwo(Mewtwo) (1, 3, 2) => MEWTWO,
	Ness(Ness) (0, 3, 2) => NESS,
	Peach(Peach) (0, 4, 3) => PEACH,
	Pikachu(Pikachu) (1, 3, 2) => PIKACHU,
	IceClimbers(IceClimbers) (3, 1, 0) => ICE_CLIMBERS,
	Jigglypuff(Jigglypuff) (1, 3, 2) => JIGGLYPUFF,
	Samus(Samus) (0, 3, 4) => SAMUS,
	Yoshi(Yoshi) (1, 0, 2) => YOSHI,
	Zelda(Zelda) (1, 3, 2) => ZELDA,
	Sheik(Sheik) (1, 3, 2) => SHEIK,
	Falco(Falco) (1, 3, 2) => FALCO,
	YoungLink(YoungLink) (1, 0, 2) => YOUNG_LINK,
	DrMario(DrMario) (1, 3, 2) => DR_MARIO,
	Roy(Roy) (1, 3, 2) => ROY,
	Pichu(Pichu) (1, 3, 2) => PICHU,
	Ganondorf(Ganondorf) (1, 3, 2) => GANONDORF,
});

pseudo_enum!(CaptainFalcon: u8 {
	0 => INDIGO,
	1 => BLACK,
	2 => RED,
	3 => WHITE,
	4 => GREEN,
	5 => BLUE,
});

pseudo_enum!(DonkeyKong: u8 {
	0 => BROWN,
	1 => BLACK,
	2 => RED,
	3 => BLUE,
	4 => GREEN,
});

pseudo_enum!(Fox: u8 {
	0 => WHITE,
	1 => RED,
	2 => BLUE,
	3 => GREEN,
});

pseudo_enum!(GameAndWatch: u8 {
	0 => BLACK,
	1 => RED,
	2 => BLUE,
	3 => GREEN,
});

pseudo_enum!(Kirby: u8 {
	0 => PINK,
	1 => YELLOW,
	2 => BLUE,
	3 => RED,
	4 => GREEN,
	5 => WHITE,
});

pseudo_enum!(Bowser: u8 {
	0 => GREEN,
	1 => RED,
	2 => BLUE,
	3 => BLACK,
});

pseudo_enum!(Link: u8 {
	0 => GREEN,
	1 => RED,
	2 => BLUE,
	3 => BLACK,
	4 => WHITE,
});

pseudo_enum!(Luigi: u8 {
	0 => GREEN,
	1 => WHITE,
	2 => BLUE,
	3 => RED, // Pink
});

pseudo_enum!(Mario: u8 {
	0 => RED,
	1 => YELLOW,
	2 => BLACK,
	3 => BLUE,
	4 => GREEN,
});

pseudo_enum!(Marth: u8 {
	0 => BLUE,
	1 => RED,
	2 => GREEN,
	3 => BLACK,
	4 => WHITE,
});

pseudo_enum!(Mewtwo: u8 {
	0 => PURPLE, // White
	1 => RED,
	2 => BLUE,
	3 => GREEN,
});

pseudo_enum!(Ness: u8 {
	0 => RED,
	1 => YELLOW,
	2 => BLUE, // Purple
	3 => GREEN,
});

pseudo_enum!(Peach: u8 {
	0 => RED,
	1 => YELLOW, // Daisy
	2 => WHITE,
	3 => BLUE,
	4 => GREEN,
});

pseudo_enum!(Pikachu: u8 {
	0 => YELLOW,
	1 => RED, // Trucker hat
	2 => BLUE, // Party hat
	3 => GREEN, // Cowboy hat
});

pseudo_enum!(IceClimbers: u8 {
	0 => BLUE,
	1 => GREEN,
	2 => ORANGE,
	3 => RED,
});

pseudo_enum!(Jigglypuff: u8 {
	0 => PINK,
	1 => RED, // Flower
	2 => BLUE, // Bow
	3 => GREEN, // Headband
	4 => YELLOW, // Crown
});

pseudo_enum!(Samus: u8 {
	0 => RED, // Orange
	1 => PINK,
	2 => BLACK,
	3 => GREEN,
	4 => BLUE, // Purple
});

pseudo_enum!(Yoshi: u8 {
	0 => GREEN,
	1 => RED,
	2 => BLUE,
	3 => YELLOW,
	4 => PINK,
	5 => CYAN, // Light blue
});

pseudo_enum!(Zelda: u8 {
	0 => PINK,
	1 => RED,
	2 => BLUE,
	3 => GREEN,
	4 => WHITE,
});

pseudo_enum!(Sheik: u8 {
	0 => NAVY,
	1 => RED,
	2 => BLUE,
	3 => GREEN,
	4 => WHITE,
});

pseudo_enum!(Falco: u8 {
	0 => TAN,
	1 => RED,
	2 => BLUE,
	3 => GREEN,
});

pseudo_enum!(YoungLink: u8 {
	0 => GREEN,
	1 => RED,
	2 => BLUE,
	3 => WHITE,
	4 => BLACK,
});

pseudo_enum!(DrMario: u8 {
	0 => WHITE,
	1 => RED,
	2 => BLUE,
	3 => GREEN,
	4 => BLACK,
});

pseudo_enum!(Roy: u8 {
	0 => PURPLE,
	1 => RED,
	2 => BLUE,
	3 => GREEN,
	4 => YELLOW,
});

pseudo_enum!(Pichu: u8 {
	0 => YELLOW,
	1 => RED,
	2 => BLUE,
	3 => GREEN,
});

pseudo_enum!(Ganondorf: u8 {
	0 => BROWN,
	1 => RED,
	2 => BLUE,
	3 => GREEN,
	4 => PURPLE,
});
