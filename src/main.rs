use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use clap::{App, Arg, ArgMatches};
use std::{
    fs::{self, File},
    io::{Read, Write},
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
    let metadata_len = metadata.len() as usize;
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
    byte_copy(
        rom_data,
        &mut memory[CHIP8_RESERVED_MEMORY_SIZE..MEMORY_SIZE],
    );
}

fn fetch_opcode(memory: &[u8], pc: u16) -> u16 {
    (memory[pc as usize] as u16) << 8 | memory[pc as usize + 1] as u16
}

enum InstructionSet {
    MachineLanguageSubroutine(u16),
    ClearScreen,
    ReturnFromSubroutine,
    JumpToAddress(u16),     // 1NNN
    ExecuteSubroutine(u16), // 2NNN
    SkipFollowingIfRegisterIsEqualToValue(u8, u8),
    SkipFollowingIfRegisterIsNotEqualToValue(u8, u8),
    SkipFollowingIfRegisterIsEqualToOtherRegister(u8, u8),
    SkipFollowingIfRegisterIsNotEqualToOtherRegister(u8, u8),
    StoreInRegister(u8, u8), // 6XNN Store number NN in register VX
    AddToRegister(u8, u8),   // 6XNN Store number NN in register VX
    CopyRegisterValueToOtherRegister(u8, u8),
    SetVxToVxOrVy(u8, u8),
    SetVxToVxAndVy(u8, u8),
    SetVxToVxXorVy(u8, u8),
    AddValueOfRegisterVyToRegisterVx(u8, u8),
    SubtractValueOfRegisterVyFromRegisterVx(u8, u8),
    StoreValueOfRegisterVyShiftedRightOneBitInVx(u8, u8),
    SetVxToValueOfVyMinusVx(u8, u8),
    StoreValueOfRegisterVyShiftedLeftOneBitInVx(u8, u8),
    StoreAddressInRegisterI(u16),
    JumpToAddressWithV0Offset(u16),
    SetVxToRandomNumberWithAMaskOf(u8, u8),
    DrawSprite(u8, u8, u8),
    StoreDelayTimerInRegisterVx(u8),
    WaitForAKeyPress(u8),
    SkipFollowingInstructionIfKeyCorrespondingToVxIsPressed(u8),
    SkipFollowingInstructionIfKeyCorrespondingToVxIsNotPressed(u8),
    SetDelayTimerToVx(u8),
    SetSoundTimerToVx(u8),
    AddVxToRegisterI(u8),
    SetIToTheMemoryAddressOfSpriteCorrespondingToVx(u8),
    StoreTheBinaryCodedDecimalEquivalentOfVx(u8),
    StoreValuesOfV0ToVxInclusiveInMemoryStartingAtAddressI(u8),
    FillRegistersV0ToVxInclusiveWithMemoryStartingAtAddressI(u8),
    None, // temp
}

fn decode_instruction(opcode: u16) -> InstructionSet {
    match opcode & 0xF000 {
        0x0000 => match opcode {
            0x00E0 => InstructionSet::ClearScreen,
            0x00EE => InstructionSet::ReturnFromSubroutine,
            _ => InstructionSet::MachineLanguageSubroutine(opcode),
        },
        0x1000 => InstructionSet::JumpToAddress(opcode & 0x0FFF),
        0x2000 => InstructionSet::ExecuteSubroutine(opcode & 0x0FFF),
        0x3000 => {
            let register_index: u8 = ((opcode & 0x0F00) >> 8) as u8;
            let value: u8 = (opcode & 0x00FF) as u8;
            InstructionSet::SkipFollowingIfRegisterIsEqualToValue(register_index, value)
        }
        0x4000 => {
            let register_index: u8 = ((opcode & 0x0F00) >> 8) as u8;
            let value: u8 = (opcode & 0x00FF) as u8;
            InstructionSet::SkipFollowingIfRegisterIsNotEqualToValue(register_index, value)
        }
        0x5000 => match opcode & 0xF00F {
            0x5000 => {
                let x_index = ((opcode & 0x0F00) >> 8) as u8;
                let y_index = ((opcode & 0x00F0) >> 4) as u8;

                InstructionSet::SkipFollowingIfRegisterIsEqualToOtherRegister(x_index, y_index)
            }
            _ => InstructionSet::None,
        },
        0x6000 => {
            let register_index: u8 = ((opcode & 0x0F00) >> 8) as u8;
            let value: u8 = (opcode & 0x00FF) as u8;
            InstructionSet::StoreInRegister(register_index, value)
        }
        0x7000 => {
            let register_index: u8 = ((opcode & 0x0F00) >> 8) as u8;
            let value: u8 = (opcode & 0x00FF) as u8;
            InstructionSet::AddToRegister(register_index, value)
        }
        0x8000 => {
            let x_index = ((opcode & 0x0F00) >> 8) as u8;
            let y_index = ((opcode & 0x00F0) >> 4) as u8;

            match opcode & 0xF00F {
                0x8000 => InstructionSet::CopyRegisterValueToOtherRegister(x_index, y_index),
                0x8001 => InstructionSet::SetVxToVxOrVy(x_index, y_index),
                0x8002 => InstructionSet::SetVxToVxAndVy(x_index, y_index),
                0x8003 => InstructionSet::SetVxToVxXorVy(x_index, y_index),
                0x8004 => InstructionSet::AddValueOfRegisterVyToRegisterVx(x_index, y_index),
                0x8005 => InstructionSet::SubtractValueOfRegisterVyFromRegisterVx(x_index, y_index),
                0x8006 => {
                    InstructionSet::StoreValueOfRegisterVyShiftedRightOneBitInVx(x_index, y_index)
                }
                0x8007 => InstructionSet::SetVxToValueOfVyMinusVx(x_index, y_index),
                0x800E => {
                    InstructionSet::StoreValueOfRegisterVyShiftedLeftOneBitInVx(x_index, y_index)
                }
                _ => InstructionSet::None,
            }
        }
        0x9000 => match opcode & 0xF00F {
            0x9000 => {
                let x_index = ((opcode & 0x0F00) >> 8) as u8;
                let y_index = ((opcode & 0x00F0) >> 4) as u8;
                InstructionSet::SkipFollowingIfRegisterIsNotEqualToOtherRegister(x_index, y_index)
            }
            _ => InstructionSet::None,
        },
        0xA000 => InstructionSet::StoreAddressInRegisterI(opcode & 0x0FFF),
        0xB000 => InstructionSet::JumpToAddressWithV0Offset(opcode & 0x0FFF),
        0xC000 => {
            let x_index = ((opcode & 0x0F00) >> 8) as u8;
            let mask = (opcode & 0x00FF) as u8;

            InstructionSet::SetVxToRandomNumberWithAMaskOf(x_index, mask)
        }
        0xD000 => {
            let x_index = ((opcode & 0x0F00) >> 8) as u8;
            let y_index = ((opcode & 0x00F0) >> 4) as u8;
            let sprite_data = (opcode & 0x000F) as u8;

            InstructionSet::DrawSprite(x_index, y_index, sprite_data)
        }
        0xE000 => {
            let x_index = ((opcode & 0x0F00) >> 8) as u8;
            match opcode & 0xF0FF {
                0xE09E => {
                    InstructionSet::SkipFollowingInstructionIfKeyCorrespondingToVxIsPressed(x_index)
                }
                0xE0A1 => {
                    InstructionSet::SkipFollowingInstructionIfKeyCorrespondingToVxIsNotPressed(
                        x_index,
                    )
                }
                _ => InstructionSet::None,
            }
        }
        0xF000 => {
            let x_index = ((opcode & 0x0F00) >> 8) as u8;

            match opcode & 0xF0FF {
                0xF007 => InstructionSet::StoreDelayTimerInRegisterVx(x_index),
                0xF00A => InstructionSet::WaitForAKeyPress(x_index),
                0xF015 => InstructionSet::SetDelayTimerToVx(x_index),
                0xF018 => InstructionSet::SetSoundTimerToVx(x_index),
                0xF01E => InstructionSet::AddVxToRegisterI(x_index),
                0xF029 => InstructionSet::SetIToTheMemoryAddressOfSpriteCorrespondingToVx(x_index),
                0xF033 => InstructionSet::StoreTheBinaryCodedDecimalEquivalentOfVx(x_index),
                0xF055 => {
                    InstructionSet::StoreValuesOfV0ToVxInclusiveInMemoryStartingAtAddressI(x_index)
                }
                0xF065 => InstructionSet::FillRegistersV0ToVxInclusiveWithMemoryStartingAtAddressI(
                    x_index,
                ),
                _ => InstructionSet::None,
            }
        }
        _ => InstructionSet::None,
    }
}

fn main() -> Result<(), String> {
    let matches = setup_cmd_program_arguments();

    let rom_path = get_rom_path(matches)?;
    println!("ROM file path you provided '{}'", rom_path);

    let rom_data = read_file_as_bytes(rom_path.as_str())?;

    let mut memory: [u8; MEMORY_SIZE] = [0 as u8; MEMORY_SIZE];
    let _v: [u8; 15];
    let _i: u16;
    let _delay_timer: u8;
    let _sound_timer: u8;
    let mut program_counter: u16 = 0x200;
    let _stack_pointer: u8;
    let _stack: [u16; 16];

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

        let opcode: u16 = fetch_opcode(&memory, program_counter);
        match decode_instruction(opcode) {
            InstructionSet::ClearScreen => {
                println!("[{:04X?}]: Clearing the Screen", opcode);
            }
            InstructionSet::JumpToAddress(address) => {
                println!("[{:04X?}]: Jump To Address [{:03X?}]", opcode, address);
            }
            InstructionSet::ExecuteSubroutine(_) => {
                println!("[{:04X?}]: Execute Subroutine", opcode);
            }
            InstructionSet::StoreInRegister(index, value) => {
                println!(
                    "[{:04X?}]: Store Value [{:02X?}] In Register [{:02X?}]",
                    opcode, value, index
                );
            }
            InstructionSet::AddToRegister(index, value) => {
                println!(
                    "[{:04X?}]: Add Value [{:02X?}] To Register [{:02X?}]",
                    opcode, value, index
                );
            }
            InstructionSet::CopyRegisterValueToOtherRegister(x, y) => {
                println!(
                    "[{:04X?}]: Copy Register [{:02X?}] Value To Other Register [{:02X?}]",
                    opcode, x, y
                );
            }
            InstructionSet::None => {
                if opcode != 0 {
                    println!("[{:04X?}]: Not Handled Opcode", opcode);
                }
            }
            InstructionSet::SkipFollowingIfRegisterIsEqualToValue(register_index, value) => {
                println!(
                    "[{:04X?}]: Skip Following If Register [{:02X?}] Is Equal To Value [{:02X?}]",
                    opcode, register_index, value,
                );
            }
            InstructionSet::SetVxToVxOrVy(x, y) => {
                println!(
                    "[{:04X?}]: Set Vx To Vx [{:02X?}] Or Vy [{:02X?}]",
                    opcode, x, y
                );
            }
            InstructionSet::SetVxToVxAndVy(x, y) => {
                println!(
                    "[{:04X?}]: Set Vx [{:02X?}] To Vx And Vy [{:02X?}]",
                    opcode, x, y
                );
            }
            InstructionSet::SetVxToVxXorVy(x, y) => {
                println!(
                    "[{:04X?}]: Set Vx [{:02X?}] To Vx Xor Vy [{:02X?}]",
                    opcode, x, y
                );
            }
            InstructionSet::SkipFollowingIfRegisterIsNotEqualToValue(register_index, value) => {
                println!(
                    "[{:04X?}]: Skip Following If Register [{:02X?}] Is Not Equal To Value [{:02X?}]",
                    opcode,
                    register_index,
                    value,
                );
            }
            InstructionSet::SkipFollowingIfRegisterIsEqualToOtherRegister(x, y) => {
                println!(
                    "[{:04X?}]: Skip Following If Register [{:02X?}] Is Equal To Other Register [{:02X?}]",
                    opcode,
                    x,
                    y
                );
            }
            InstructionSet::AddValueOfRegisterVyToRegisterVx(x, y) => {
                println!(
                    "[{:04X?}]: Add Value Of Register Vy [{:02X?}] To Register Vx [{:02X?}]",
                    opcode, y, x
                );
            }
            InstructionSet::MachineLanguageSubroutine(_) => {
                if opcode != 0 {
                    println!("[{:04X?}]: Machine Language Subroutine", opcode);
                }
            }
            InstructionSet::StoreAddressInRegisterI(address) => {
                println!(
                    "[{:04X?}]: Store Address [{:03X?}] In Register I",
                    opcode, address
                );
            }
            InstructionSet::JumpToAddressWithV0Offset(address) => {
                println!(
                    "[{:04X?}]: Jump To Address [{:03X?}] mWith V0 Offset",
                    opcode, address
                );
            }
            InstructionSet::SetVxToRandomNumberWithAMaskOf(x, mask) => {
                println!(
                    "[{:04X?}]: Set Vx [{:02X?}] To Random Number With A Mask Of [{:02X?}]",
                    opcode, x, mask
                );
            }
            InstructionSet::DrawSprite(x, y, sprite_data) => {
                println!("[{:04X?}]: Draw a sprite at position VX [{:02X?}], VY [{:02X?}] with N [{:02X?}] bytes of sprite data starting at the address stored in I", opcode, x, y, sprite_data);
            }
            InstructionSet::ReturnFromSubroutine => {
                println!("[{:04X?}]: Return From Subroutine", opcode);
            }
            InstructionSet::SkipFollowingIfRegisterIsNotEqualToOtherRegister(x, y) => {
                println!(
                    "[{:04X?}]: Skip Following If Register [{:02X?}] Is Not Equal To Other Register  [{:02X?}]",
                    opcode,
                    x,
                    y
                );
            }
            InstructionSet::SubtractValueOfRegisterVyFromRegisterVx(x, y) => {
                println!(
                    "[{:04X?}]: Subtract Value Of Register Vy [{:02X?}] From Register Vx [{:02X?}]",
                    opcode, y, x,
                );
            }
            InstructionSet::StoreValueOfRegisterVyShiftedRightOneBitInVx(x, y) => {
                println!(
                    "[{:04X?}]: Store Value Of Register Vy [{:02X?}] Shifted Right One Bit In Vx [{:02X?}]",
                    opcode,
                    y,
                    x,
                );
            }
            InstructionSet::SetVxToValueOfVyMinusVx(x, y) => {
                println!(
                    "[{:04X?}]: Set Vx [{:02X?}] To Value Of Vy [{:02X?}] Minus Vx",
                    opcode, x, y
                );
            }
            InstructionSet::StoreValueOfRegisterVyShiftedLeftOneBitInVx(x, y) => {
                println!(
                    "[{:04X?}]: Store Value Of Register Vy [{:02X?}] Shifted Left One Bit In Vx [{:02X?}]",
                    opcode,
                    y,
                    x,
                );
            }
            InstructionSet::StoreDelayTimerInRegisterVx(x) => {
                println!(
                    "[{:04X?}]: Store Delay Timer In Register Vx [{:02X?}]",
                    opcode, x
                );
            }
            InstructionSet::WaitForAKeyPress(x) => {
                println!(
                    "[{:04X?}]: Wait For A Key Press And Store The Result In Register VX [{:02X?}]",
                    opcode, x
                );
            }
            InstructionSet::SkipFollowingInstructionIfKeyCorrespondingToVxIsPressed(x) => {
                println!(
                    "[{:04X?}]: Skip Following Instruction If Key Corresponding To Vx [{:02X?}] Is Pressed",
                    opcode,
                    x
                );
            }
            InstructionSet::SkipFollowingInstructionIfKeyCorrespondingToVxIsNotPressed(x) => {
                println!(
                    "[{:04X?}]: kip Following Instruction If Key Corresponding To Vx [{:02X?}] Is Not Pressed",
                    opcode,
                    x
                );
            }
            InstructionSet::SetDelayTimerToVx(x) => {
                println!("[{:04X?}]: Set Delay Timer To Vx [{:02X?}]", opcode, x);
            }
            InstructionSet::SetSoundTimerToVx(x) => {
                println!("[{:04X?}]: Set Sound Timer To Vx [{:02X?}]", opcode, x);
            }
            InstructionSet::AddVxToRegisterI(x) => {
                println!("[{:04X?}]: Add Vx [{:02X?}] To Register I", opcode, x);
            }
            InstructionSet::SetIToTheMemoryAddressOfSpriteCorrespondingToVx(x) => {
                println!(
                    "[{:04X?}]: Set I To The Memory Address Of Sprite Corresponding To Vx [{:02X?}]",
                    opcode, x
                );
            }
            InstructionSet::StoreTheBinaryCodedDecimalEquivalentOfVx(x) => {
                println!(
                    "[{:04X?}]: Store The Binary-Coded Decimal Equivalent Of Vx [{:02X?}]",
                    opcode, x
                );
            }
            InstructionSet::StoreValuesOfV0ToVxInclusiveInMemoryStartingAtAddressI(x) => {
                println!(
                    "[{:04X?}]: Store Values Of V0 To Vx [{:02X?}] Inclusive In Memory Starting At Address I",
                    opcode, x
                );
            }
            InstructionSet::FillRegistersV0ToVxInclusiveWithMemoryStartingAtAddressI(x) => {
                println!(
                    "[{:04X?}]: Fill Registers V0 To Vx [{:02X?}] Inclusive With Memory Starting At Address I",
                    opcode, x
                );
            }
        }

        program_counter += 2;

        canvas.clear();
        canvas.present();
    }

    Ok(())
}
