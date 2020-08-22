use ggez::conf;
use ggez::event::{self};

mod debug;
mod chip8_state;
mod setup;
mod utils;
mod instructions;
mod fonts_sprites;
mod write_to_memory;
mod test;
mod main_state;

use utils::read_file_as_bytes;
use chip8_state::*;
use setup::*;
use main_state::*;

fn main() -> ggez::GameResult {
    let matches = setup_cmd_program_arguments();

    let rom_path = get_rom_path(matches).unwrap();
    println!("ROM file path you provided '{}'", rom_path);

    let rom_data = read_file_as_bytes(rom_path.as_str()).unwrap();

    let cb = ggez::ContextBuilder::new("CHIP-8 VM", "ggez")
        .window_setup(conf::WindowSetup::default().title("CHIP-8 VM"))
        .window_mode(conf::WindowMode::default().resizable(false).dimensions(
            (DISPLAY_SIZE[0] * SCALE) as f32
                + if cfg!(debug_assertions) {
                    DEBUG_EXTRA_DISPLAY_SIZE[0]
                } else {
                    0.0
                },
            (DISPLAY_SIZE[1] * SCALE) as f32
                + if cfg!(debug_assertions) {
                    DEBUG_EXTRA_DISPLAY_SIZE[1]
                } else {
                    0.0
                },
        ));
    let (ref mut ctx, event_loop) = &mut cb.build()?;

    let hidpi_factor = event_loop.get_primary_monitor().get_hidpi_factor() as f32;

    let state = &mut MainState::new(ctx, hidpi_factor, &rom_data)?;

    event::run(ctx, event_loop, state)
}
