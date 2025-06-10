use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use anyhow::Result;
use bytemuck::cast_slice;

fn read_str<'a>(buf: &'a mut [u8], rom: &mut File) -> Result<&'a str> {
    rom.read_exact(buf)?;
    Ok(str::from_utf8(buf)?.trim_end_matches('\0'))
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let mut rom = File::open(&args[1])?;

    let mut game_title = [0u8; 12];
    let game_title = read_str(&mut game_title, &mut rom)?;

    let mut gamecode = [0u8; 4];
    let gamecode = read_str(&mut gamecode, &mut rom)?;

    let mut makercode = [0u8; 2];
    let makercode = read_str(&mut makercode, &mut rom)?;

    println!("Game Title: {game_title}");
    println!("Gamecode: {gamecode}");
    println!("Makercode: {makercode}");

    rom.seek(SeekFrom::Start(0x68))?;

    let mut title_offset = [0u8; 4];
    rom.read_exact(&mut title_offset)?;
    let title_offset = u32::from_le_bytes(title_offset);

    rom.seek(SeekFrom::Start(title_offset as _))?;
    rom.seek_relative(0x240)?;
    let mut title = [0u8; 100];
    rom.read_exact(&mut title)?;
    let title = String::from_utf16(cast_slice(&title))?;

    println!("-------Title-------");
    println!("{}", title.trim_end_matches('\0'));
    println!("-------------------");

    Ok(())
}

// pub struct Title {}

// fn parse_title() -> Result<()> {
//     Ok(())
// }
