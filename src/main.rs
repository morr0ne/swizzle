use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use anyhow::Result;
use bytemuck::cast_slice;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let mut rom = File::open(&args[1])?;

    let mut name = [0u8; 12];
    rom.read_exact(&mut name)?;
    let name = str::from_utf8(&name)?.trim_end_matches('\0');

    println!("Name: {name}");

    rom.seek(SeekFrom::Start(0x68))?;

    let mut title_offset = [0u8; 4];
    rom.read_exact(&mut title_offset)?;
    let title_offset = u32::from_le_bytes(title_offset);

    rom.seek(SeekFrom::Start(title_offset as _))?;
    rom.seek_relative(0x240)?;
    let mut title = [0u8; 100];
    rom.read_exact(&mut title)?;
    let title = String::from_utf16(cast_slice(&title))?;

    println!("Title: {}", title.trim_end_matches('\0'));

    Ok(())
}
