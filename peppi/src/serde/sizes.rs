use crate::model::slippi::Version;

/// The size in bytes of the Game Start event payload (excluding command byte)
#[derive(num_enum::IntoPrimitive)]
#[repr(usize)]
pub enum GameStart {
	V0_1_0 = 58,
	V1_2_0 = 59,
	V1_4_0 = 63,
}

impl GameStart {
	fn try_from(version: Version) -> Option<Self> {}
}

#[derive(num_enum::IntoPrimitive)]
#[repr(usize)]
pub enum FramePre {
	V0_1_0 = 58,
	V1_2_0 = 59,
	V1_4_0 = 63,
}

#[derive(num_enum::IntoPrimitive)]
#[repr(usize)]
pub enum FramePost {
	V0_1_0 = 33,
	V0_2_0 = 37,
	V2_0_0 = 51,
	V2_1_0 = 52,
	V3_5_0 = 72,
	V3_8_0 = 76,
	V3_11_0 = 80,
}

#[derive(num_enum::IntoPrimitive)]
#[repr(usize)]
pub enum GameEnd {
	V0_1_0 = 1,
	V2_0_0 = 2,
	V3_13_0 = 6,
}

#[derive(num_enum::IntoPrimitive)]
#[repr(usize)]
pub enum FrameStart {
	V2_2_0 = 8,
	V3_10_0 = 12,
}

#[derive(num_enum::IntoPrimitive)]
#[repr(usize)]
pub enum Item {
	V3_0_0 = 37,
	V3_2_0 = 41,
	V3_6_0 = 42,
}

#[derive(num_enum::IntoPrimitive)]
#[repr(usize)]
pub enum FrameEnd {
	V3_0_0 = 4,
	V3_7_0 = 8,
}
