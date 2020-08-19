use ggez::conf;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::{Context, GameResult};

mod debug;
use debug::ImGuiWrapper;

mod chip8_state;
use chip8_state::*;

mod setup;
use setup::*;

mod utils;
use utils::*;

mod instructions;
use instructions::*;

mod fonts_sprites;
use fonts_sprites::FONTS_SPRITES;

mod test;

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

struct MainState {
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
    chip8_state: Chip8State,
    debug_run_next: bool,
}

impl MainState {
    fn new(mut ctx: &mut Context, hidpi_factor: f32, rom_data: &[u8]) -> GameResult<MainState> {
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx);
        let mut s = MainState {
            debug_run_next: if cfg!(debug_assertions) { false } else { true },
            imgui_wrapper,
            hidpi_factor,
            chip8_state: Chip8State {
                waiting_for_key_press: false,
                key_index_store: 0x00,
                display_data: [false; DISPLAY_SIZE[0] * DISPLAY_SIZE[1]],
                memory: [0 as u8; MEMORY_SIZE],
                v: [0 as u8; 16],
                i: 0,
                delay_timer: 0,
                sound_timer: 0,
                program_counter: 0x200,
                stack_pointer: 0,
                stack: [0 as u16; 16],
                chip8_key: None,
            },
        };

        write_font_data_to_memory(&mut s.chip8_state.memory);
        write_rom_data_to_memory(&mut s.chip8_state.memory, rom_data);

        Ok(s)
    }
}

fn write_font_data_to_memory(memory: &mut [u8; MEMORY_SIZE]) {
    for i in 0..FONTS_SPRITES.len() {
        for j in 0..FONTS_SPRITES[i].len() {
            memory[i * FONTS_SPRITES[i].len() + j] = FONTS_SPRITES[i][j];
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            if self.chip8_state.waiting_for_key_press {
                if let Some(code) = self.chip8_state.chip8_key {
                    self.chip8_state.v[self.chip8_state.key_index_store as usize] = code;
                    self.chip8_state.waiting_for_key_press = false;
                }
            } else if self.debug_run_next {
                let opcode =
                    fetch_opcode(&self.chip8_state.memory, self.chip8_state.program_counter)
                        .unwrap();
                let instruction = decode_opcode(opcode);

                if cfg!(debug_assertions) && opcode != 0 {
                    println!("[{:04X?}]: {}", opcode, instruction.to_string());
                }
                self.chip8_state.execute_instruction(instruction, opcode);
                if let Some(code) = self.chip8_state.chip8_key {
                    //println!("Key Pressed: {:02X?}", code);
                }
                if self.chip8_state.delay_timer > 0 {
                    self.chip8_state.delay_timer -= 1;
                }
                if self.chip8_state.sound_timer > 0 {
                    self.chip8_state.sound_timer -= 1;
                }
                self.chip8_state.program_counter += 2;

                if cfg!(debug_assertions) {
                    self.debug_run_next = false;
                }
            }
        }

        //println!("FPS: {}", ggez::timer::fps(ctx));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Render game stuff
        {
            for x in 0..DISPLAY_SIZE[0] {
                for y in 0..DISPLAY_SIZE[1] {
                    if self.chip8_state.display_data[y * DISPLAY_SIZE[0] + x] {
                        let rect = graphics::Rect::new(
                            (x * SCALE) as f32,
                            (y * SCALE) as f32,
                            (SCALE) as f32,
                            (SCALE) as f32,
                        );
                        let r = graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::Fill(graphics::FillOptions::DEFAULT),
                            rect,
                            graphics::Color::new(1.0, 1.0, 1.0, 1.0),
                        )?;
                        graphics::draw(ctx, &r, graphics::DrawParam::default())?;
                    }
                }
            }
        }

        // Render game ui
        if cfg!(debug_assertions) {
            self.imgui_wrapper
                .render(ctx, self.hidpi_factor, &self.chip8_state);
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        if cfg!(debug_assertions) {
            self.imgui_wrapper.update_mouse_pos(x, y);
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if cfg!(debug_assertions) {
            self.imgui_wrapper.update_mouse_down((
                button == MouseButton::Left,
                button == MouseButton::Right,
                button == MouseButton::Middle,
            ));
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if cfg!(debug_assertions) {
            self.imgui_wrapper.update_mouse_down((false, false, false));
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Space {
            self.debug_run_next = true;
        }

        self.chip8_state.chip8_key = match keycode {
            KeyCode::Key1 => Some(0x1),
            KeyCode::Key2 => Some(0x2),
            KeyCode::Key3 => Some(0x3),
            KeyCode::Key4 => Some(0xC),
            KeyCode::Q => Some(0x4),
            KeyCode::W => Some(0x5),
            KeyCode::E => Some(0x6),
            KeyCode::R => Some(0xD),
            KeyCode::A => Some(0x7),
            KeyCode::S => Some(0x8),
            KeyCode::D => Some(0x9),
            KeyCode::F => Some(0xE),
            KeyCode::Z => Some(0xA),
            KeyCode::X => Some(0x0),
            KeyCode::C => Some(0xB),
            KeyCode::V => Some(0xF),
            _ => None,
        };
        if cfg!(debug_assertions) {
            self.imgui_wrapper.update_key_down(keycode, keymods);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.chip8_state.chip8_key = None;

        if cfg!(debug_assertions) {
            self.imgui_wrapper.update_key_up(keycode, keymods);
        }
    }

    fn text_input_event(&mut self, _ctx: &mut Context, val: char) {
        if cfg!(debug_assertions) {
            self.imgui_wrapper.update_text(val);
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        if cfg!(debug_assertions) {
            self.imgui_wrapper.update_scroll(x, y);
        }
    }
}

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
