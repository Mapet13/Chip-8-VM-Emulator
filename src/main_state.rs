use crate::{chip8_state::*, debug::ImGuiWrapper, instructions::decode_opcode};
use ggez::{
    event::{EventHandler, KeyCode, KeyMods, MouseButton},
    graphics, Context, GameResult,
};

fn fetch_opcode(memory: &[u8], pc: u16) -> Result<u16, ()> {
    if pc >= MEMORY_SIZE as u16 {
        return Err(());
    }

    Ok((memory[pc as usize] as u16) << 8 | memory[pc as usize + 1] as u16)
}

pub struct MainState {
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
    chip8_state: Chip8VM,
    debug_run_next: bool,
}

impl MainState {
    pub fn new(mut ctx: &mut Context, hidpi_factor: f32, rom_data: &[u8]) -> GameResult<MainState> {
        Ok(MainState {
            debug_run_next: if cfg!(debug_assertions) { false } else { true },
            imgui_wrapper: ImGuiWrapper::new(&mut ctx),
            hidpi_factor,
            chip8_state: Chip8VM::new(rom_data),
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const FPS: u32 = 400;

        while ggez::timer::check_update_time(ctx, FPS) {
            if self.chip8_state.waiting_for_key_press {
                if let Some(code) = self.chip8_state.pressed_key {
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
                if let Some(_code) = self.chip8_state.pressed_key {
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

        self.chip8_state.handle_keyboard_input(keycode);

        if cfg!(debug_assertions) {
            self.imgui_wrapper.update_key_down(keycode, keymods);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.chip8_state.pressed_key = None;

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
