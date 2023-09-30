use std::io::{Cursor, Result};
use crate::{
	model::{
		game::{self, Start, End, NUM_PORTS}, metadata, frame,
	},
    serde::de::{self, Event, expect_bytes, LastCharStates},
};

use byteorder::ReadBytesExt;
type BE = byteorder::BigEndian;

pub struct DataOutline<'a> {
    pub pre: &'a [u8],
    pub post: &'a [u8],
}

impl<'a> TryFrom<DataOutline<'a>> for frame::Data {
    type Error = std::io::Error;

    fn try_from(outline: DataOutline<'a>) -> Result<Self> {
        let mut r = outline.post;
        let mut lcs:LastCharStates = Default::default();
        let post = de::frame_post(&mut r, &mut lcs)?.event;
        r = outline.pre;
        let pre = de::frame_pre(&mut r, &lcs)?.event;
        Ok(frame::Data {
            pre,
            post,
        })
    }
}

#[derive(Clone, Debug)]
pub struct GameInfo {
    pub start: game::Start,
    pub end: game::End,
    pub metadata: Option<metadata::Metadata>,
	pub metadata_raw: Option<serde_json::Map<String, serde_json::Value>>,
}

// assume pre/post/item go together
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameOutline {
    pub index: i32,
    pub offset: u32,
    pub num_items: u8,
    // pub pre_offset: u32,
    // pub post_offset: u32,
    // pub start_offset: Option<u32>,
    // pub end_offset: Option<u32>,
    // pub item_offset: Option<u32>,
    // pub num_items: Option<u8>,
    // pub start: Option<&'a [u8]>,
    // pub end: Option<&'a [u8]>,
    // pub pre_leaders: [Option<&'a [u8]>; NUM_PORTS],
    // pub pre_follower: [Option<&'a [u8]>; NUM_PORTS],
    // pub post_leaders: [Option<&'a [u8]>; NUM_PORTS],
    // pub post_follower: [Option<&'a [u8]>; NUM_PORTS],
    // pub items: [Option<&'a [u8]>; 15],
}

#[derive(Clone, Debug)]
pub struct GameOutline {
    pub info: GameInfo,
    pub gecko_offset: u32,
    pub frames: Vec<FrameOutline>,
}

pub fn outline<'a>(buf: &'a [u8]) -> Result<GameOutline> {
    const RAW_OFFSET: usize = std::mem::size_of::<crate::SLIPPI_FILE_SIGNATURE>() + 4;
    let mut r = Cursor::new(buf);

	expect_bytes(&mut r, &crate::SLIPPI_FILE_SIGNATURE)?;
	let raw_len = r.read_u32::<BE>()? as usize;

	let (_, payload_sizes) = de::payload_sizes(&mut r)?;
    let game_start_size = payload_sizes[Event::GameStart as usize]
        .ok_or_else(|| err!("Missing Game Start payload size"))? as usize;
    let game_end_size = payload_sizes[Event::GameEnd as usize]
        .ok_or_else(|| err!("Missing Game End payload size"))? as usize;

    if r.read_u8()? != de::Event::GameStart as u8 {
        return Err(err!("First event is not Game Start. Found {}", code));
    }
    let start = de::game_start(&mut r)?;

    let mut has_game_end = false;
    let mut frames: Vec<FrameOutline> = Vec::new();
    while raw_len == 0 || r.position() < raw_len {
        let code = r.read_u8()?;
        if code == de::Event::GameEnd as u8 {
            has_game_end = true;
            break;
        }
    }

    let end = has_game_end.then(|| {
        de::game_end(&mut r)?;
    });

    bytes_read += game_end_size as usize + 1;

    todo!()
}

fn take<'a>(cursor: &mut Cursor<&'a [T]>, n: u64) -> Result<&'a [T]> {
    let slc = cursor.get_ref();

}
