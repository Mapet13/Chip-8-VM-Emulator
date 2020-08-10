use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use clap::{App, Arg, ArgMatches};
use std::{
    fs::{self, File}, io::{Write, Read}
};

const MEMORY_SIZE: usize = 0x1000; // 4096

const CHIP8_RESERVED_MEMORY_SIZE: usize = 0x200; // 512

const SCALE: usize = 16;
const DISPLAY_SIZE: [usize; 2] = [64, 32];

fn setup_cmd_program_arguments() -> ArgMatches<'static> {
    App::new("Chip-8 VM emulator")
        .version("0.1.0")
        .author("Jakub Sordyl 'Mapet13' <jakubsordyl1@gmail.com>")
        .about("This is a simple Chip-8 VM emulator developed in Rust-lang for learning purpose.")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .required(true)
                .takes_value(true)
                .help("The ROM file you want to run in this VM"),
        )
        .get_matches()
}

fn get_rom_path(matches: ArgMatches) -> Result<String, String> {
    match matches.value_of("file") {
        Some(value) => Ok(value.to_string()),
        None => Err("Command line argument error".to_string()),
    }
}

fn read_file_as_bytes(filename: &str) -> Result<Vec<u8>, String> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let metadata_len =  metadata.len() as usize;
    if metadata_len > MEMORY_SIZE - CHIP8_RESERVED_MEMORY_SIZE {
        return Err("File is to big".to_string());
    }
    let mut buffer = vec![0; metadata_len];
    match f.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(_) => Err("Error with reading ROM file".to_string()),
    }
}

fn byte_copy(from: &[u8], mut to: &mut [u8]) -> usize {
    to.write(from).unwrap()
}

fn write_rom_data_to_memory(memory: &mut [u8; MEMORY_SIZE], rom_data: &[u8]) {
    byte_copy(rom_data, &mut memory[CHIP8_RESERVED_MEMORY_SIZE..MEMORY_SIZE]);
}

fn main() -> Result<(), String> {
    let matches = setup_cmd_program_arguments();

    let rom_path = get_rom_path(matches)?;
    println!("ROM file path you provided '{}'", rom_path);

    let rom_data = read_file_as_bytes(rom_path.as_str())?;

    let mut memory: [u8; MEMORY_SIZE] = [0 as u8; MEMORY_SIZE];
    let _v: [u8; 15];
    let _i: u16;

    write_rom_data_to_memory(&mut memory, &rom_data);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "rust-sdl2 demo: Video",
            (DISPLAY_SIZE[0] * SCALE) as u32,
            (DISPLAY_SIZE[1] * SCALE) as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.clear();
        canvas.present();
    }

    Ok(())
}
