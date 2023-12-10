# Peppi

Peppi is a Rust parser for [.slp](https://github.com/project-slippi/slippi-wiki/blob/master/SPEC.md) game replay files for [Super Smash Brothers Melee](https://en.wikipedia.org/wiki/Super_Smash_Bros._Melee) for the Nintendo GameCube. These replays are generated by Jas Laferriere's [Slippi](https://github.com/JLaferri/project-slippi) recording code, which runs on a Wii or the [Dolphin](https://dolphin-emu.org/) emulator.

## Installation

In your `Cargo.toml`:

```toml
[dependencies]
peppi = "2.0.0-alpha.2"
```

## Usage

One-shot parsing just requires calling `peppi::game`.

```rust
use std::{fs, io};

// you can optionally use the `ssbm-data` crate for enums
use ssbm_data::action_state::Common::{self, *};

fn main() {
    let mut r = io::BufReader::new(fs::File::open("game.slp").unwrap());
    let game = peppi::game(&mut r, None).unwrap();

    // print general info about the game
    println!("{:#?}", game);

    // find the frames on which each player died
    let mut is_dead: Vec<_> = game.frames.ports.iter().map(|_| false).collect();
    for frame_idx in 0..game.frames.id.len() {
        for (port_idx, port_data) in game.frames.ports.iter().enumerate() {
            match port_data
                .leader
                .post
                .state
                .get(frame_idx)
                .and_then(|s| Common::try_from(s).ok())
            {
                Some(DeadDown)
                | Some(DeadLeft)
                | Some(DeadRight)
                | Some(DeadUp)
                | Some(DeadUpStar)
                | Some(DeadUpStarIce)
                | Some(DeadUpFall)
                | Some(DeadUpFallHitCamera)
                | Some(DeadUpFallHitCameraFlat)
                | Some(DeadUpFallIce)
                | Some(DeadUpFallHitCameraIce) => {
                    if !is_dead[port_idx] {
                        is_dead[port_idx] = true;
                        println!(
                            "{} died on frame {}",
                            game.start.players[port_idx].port,
                            game.frames.id.get(frame_idx).unwrap(),
                        )
                    }
                }
                _ => is_dead[port_idx] = false,
            }
        }
    }
}
```

For real-time parsing, you can "drive" things yourself:

```rust
use std::fs;
use std::io::BufReader;
use byteorder::ReadBytesExt;
use peppi::io::slippi::de;

fn main() {
    let mut r = BufReader::new(fs::File::open("v3.12.slp").unwrap());

    // UBJSON wrapper (skip if using spectator protocol)
    let size = de::parse_header(&mut r, None).unwrap() as usize;

    // payload sizes & game start
    let mut state = de::parse_start(&mut r, None).unwrap();

    // loop until we hit GameEnd or run out of bytes
    while de::parse_event(&mut r, &mut state, None).unwrap() != de::Event::GameEnd as u8
        && state.bytes_read() < size
    {
        println!(
            "current frame number: {:?}",
            state.frames().id.iter().last()
        );
    }

    // `U` (0x55) means metadata next (skip if using spectator protocol)
    if r.read_u8().unwrap() == 0x55 {
        de::parse_metadata(&mut r, &mut state, None).unwrap();
    }
}
```

## Inspector

⚠ The `slp` tool has moved to the [peppi-slp](https://github.com/hohav/peppi-slp) crate.
