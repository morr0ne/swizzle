use std::fs::{self, File};

use anyhow::{Result, bail};
use bytemuck::cast_slice;

fn main() -> Result<()> {


    let name = &rom[0..12];
    let name = str::from_utf8(&[
        b'P', b'A', b'L', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ])?
    .trim_end_matches('\0');

    println!("Name: {}", name);

    dbg!(name.as_bytes());

    let title_offset = &rom[0x68..0x68 + 4];

    let title_offset = u32::from_le_bytes([
        title_offset[0],
        title_offset[1],
        title_offset[2],
        title_offset[3],
    ]) as usize;

    if title_offset == 0 {
        bail!("WAAAAAA")
    }

    let title_header = &rom[title_offset..];

    let title = &title_header[0x240..0x240 + 100];

    println!("Title: {}", String::from_utf16_lossy(cast_slice(title)));

    Ok(())
}
