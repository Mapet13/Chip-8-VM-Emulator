use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod setup;
use setup::*;

mod utils;
use utils::*;

mod instructions;
use instructions::*;

const MEMORY_SIZE: usize = 0x1000; // 4096
const CHIP8_RESERVED_MEMORY_SIZE: usize = 0x200; // 512
const SCALE: usize = 16;
const DISPLAY_SIZE: [usize; 2] = [64, 32];

fn write_rom_data_to_memory(memory: &mut [u8; MEMORY_SIZE], rom_data: &[u8]) {
    byte_copy(
        rom_data,
        &mut memory[CHIP8_RESERVED_MEMORY_SIZE..MEMORY_SIZE],
    );
}

fn fetch_opcode(memory: &[u8], pc: u16) -> Result<u16, ()> {
    if pc >= MEMORY_SIZE as u16 {
        return Err(());
    }

    Ok((memory[pc as usize] as u16) << 8 | memory[pc as usize + 1] as u16)
}

fn main() -> Result<(), String> {
    let matches = setup_cmd_program_arguments();

    let rom_path = get_rom_path(matches)?;
    println!("ROM file path you provided '{}'", rom_path);

    let rom_data = read_file_as_bytes(rom_path.as_str(), MEMORY_SIZE - CHIP8_RESERVED_MEMORY_SIZE)?;

    let mut memory: [u8; MEMORY_SIZE] = [0 as u8; MEMORY_SIZE];
    let mut v: [u8; 16] = [0 as u8; 16];
    let mut i: u16 = 0;
    let _delay_timer: u8;
    let _sound_timer: u8;
    let mut program_counter: u16 = 0x200;
    let mut stack_pointer: u8 = 0;
    let mut stack: [u16; 16] = [0 as u16; 16];

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

        let opcode;
        match fetch_opcode(&memory, program_counter) {
            Err(_) => break 'running,
            Ok(x) => opcode = x,
        }
        let instruction = decode_opcode(opcode);

        match instruction {
            InstructionSet::ClearScreen => {}
            InstructionSet::ReturnFromSubroutine => {
                stack_pointer -= 1;
                program_counter = stack[stack_pointer as usize];      
            }
            InstructionSet::JumpToAddress(address) => program_counter = address - 2,
            InstructionSet::ExecuteSubroutine(address) => {
                stack[stack_pointer as usize] = program_counter;
                stack_pointer += 1;
                program_counter = address - 2;
            }
            InstructionSet::AddToRegister(index, value) => v[index as usize] += value,
            InstructionSet::StoreInRegister(index, value) => v[index as usize] = value,
            InstructionSet::AddVxToRegisterI(index) => i += v[index as usize] as u16,
            InstructionSet::CopyRegisterValueToOtherRegister(x, y) => v[x as usize] = v[y as usize],
            InstructionSet::SkipFollowingIfRegisterIsEqualToValue(index, value) => {
                if v[index as usize] == value {
                    program_counter += 2;
                }
            }
            InstructionSet::SkipFollowingIfRegisterIsNotEqualToValue(index, value) => {
                if v[index as usize] != value {
                    program_counter += 2;
                }
            }
            InstructionSet::SkipFollowingIfRegisterIsEqualToOtherRegister(x, y) => {
                if v[x as usize] == v[y as usize] {
                    program_counter += 2;
                }
            }
            InstructionSet::SetVxToVxOrVy(x, y) => v[x as usize] |= v[y as usize],
            InstructionSet::SetVxToVxAndVy(x, y) => v[x as usize] &= v[y as usize],
            InstructionSet::SetVxToVxXorVy(x, y) => v[x as usize] ^= v[y as usize],
            InstructionSet::AddValueOfRegisterVyToRegisterVx(x, y) => {
                let sum = v[x as usize] as u16 + v[y as usize] as u16;
                v[0xF] = if sum > 255 { 1 } else { 0 };
                v[x as usize] = (sum & 0x00FF) as u8;
            }
            InstructionSet::SubtractValueOfRegisterVyFromRegisterVx(x, y) => {
                v[0xF] = if v[x as usize] > v[y as usize] { 1 } else { 0 };
                v[x as usize] -= v[y as usize];
            }
            InstructionSet::StoreValueOfRegisterVyShiftedRightOneBitInVx(x, y) => {
                v[x as usize] = v[y as usize] >> 1;
                v[0xF] = v[x as usize] & 0x01;
            }
            InstructionSet::SetVxToValueOfVyMinusVx(x, y) => {
                v[0xF] = if v[y as usize] > v[x as usize] { 1 } else { 0 };
                v[x as usize] = v[y as usize] - v[x as usize];
            }
            InstructionSet::StoreValueOfRegisterVyShiftedLeftOneBitInVx(x, y) => {
                v[x as usize] = v[y as usize] << 1;
                v[0xF] = (v[x as usize] >> 7) & 0x01;
            }
            InstructionSet::SkipFollowingIfRegisterIsNotEqualToOtherRegister(x, y) => {
                if v[x as usize] != v[y as usize] {
                    program_counter += 2;
                }
            },
            _ => {}
        }

        if opcode != 0 {
            println!("[{:04X?}]: {}", opcode, instruction.to_string());
        }

        program_counter += 2;

        canvas.clear();
        canvas.present();
    }

    Ok(())
}
