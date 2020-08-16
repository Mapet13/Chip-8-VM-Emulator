// FROM: https://raw.githubusercontent.com/iolivia/imgui-ggez-starter/master/src/imgui_wrapper.rs
// AUTHOR: Olivia Ifrim
// MODIFY BY: Mapet13

use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::Context;

use gfx_core::{handle::RenderTargetView, memory::Typed};
use gfx_device_gl;

use imgui::*;
use imgui_gfx_renderer::*;

use std::time::Instant;

use super::chip8_state::*;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
    wheel_h: f32,
}

pub struct ImGuiWrapper {
    pub imgui: imgui::Context,
    pub renderer: Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,
    last_frame: Instant,
    mouse_state: MouseState,
}

impl ImGuiWrapper {
    pub fn new(ctx: &mut Context) -> Self {
        // Create the imgui object
        let mut imgui = imgui::Context::create();
        let (factory, gfx_device, _, _, _) = graphics::gfx_objects(ctx);

        // Shaders
        let shaders = {
            let version = gfx_device.get_info().shading_language;
            if version.is_embedded {
                if version.major >= 3 {
                    Shaders::GlSlEs300
                } else {
                    Shaders::GlSlEs100
                }
            } else if version.major >= 4 {
                Shaders::GlSl400
            } else if version.major >= 3 {
                Shaders::GlSl130
            } else {
                Shaders::GlSl110
            }
        };

        // Renderer
        let mut renderer = Renderer::init(&mut imgui, &mut *factory, shaders).unwrap();

        {
            let mut io = imgui.io_mut();
            io[Key::Tab] = KeyCode::Tab as _;
            io[Key::LeftArrow] = KeyCode::Left as _;
            io[Key::RightArrow] = KeyCode::Right as _;
            io[Key::UpArrow] = KeyCode::Up as _;
            io[Key::DownArrow] = KeyCode::Down as _;
            io[Key::PageUp] = KeyCode::PageUp as _;
            io[Key::PageDown] = KeyCode::PageDown as _;
            io[Key::Home] = KeyCode::Home as _;
            io[Key::End] = KeyCode::End as _;
            io[Key::Insert] = KeyCode::Insert as _;
            io[Key::Delete] = KeyCode::Delete as _;
            io[Key::Backspace] = KeyCode::Back as _;
            io[Key::Space] = KeyCode::Space as _;
            io[Key::Enter] = KeyCode::Return as _;
            io[Key::Escape] = KeyCode::Escape as _;
            io[Key::KeyPadEnter] = KeyCode::NumpadEnter as _;
            io[Key::A] = KeyCode::A as _;
            io[Key::C] = KeyCode::C as _;
            io[Key::V] = KeyCode::V as _;
            io[Key::X] = KeyCode::X as _;
            io[Key::Y] = KeyCode::Y as _;
            io[Key::Z] = KeyCode::Z as _;
        }

        // Create instance
        Self {
            imgui,
            renderer,
            last_frame: Instant::now(),
            mouse_state: MouseState::default(),
        }
    }

    pub fn render(&mut self, ctx: &mut Context, hidpi_factor: f32, chip8_state: &Chip8State) {
        // Update mouse
        self.update_mouse();

        // Create new frame
        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        let (draw_width, draw_height) = graphics::drawable_size(ctx);
        self.imgui.io_mut().display_size = [draw_width, draw_height];
        self.imgui.io_mut().display_framebuffer_scale = [hidpi_factor, hidpi_factor];
        self.imgui.io_mut().delta_time = delta_s;

        let ui = self.imgui.frame();

        let flags = imgui::WindowFlags::NO_RESIZE
            | imgui::WindowFlags::NO_MOVE
            | imgui::WindowFlags::NO_COLLAPSE;

        // Various ui things
        {
            Window::new(im_str!("Registers"))
                .size(
                    [
                        DEBUG_EXTRA_DISPLAY_SIZE[0],
                        (DISPLAY_SIZE[1] * SCALE) as f32,
                    ],
                    imgui::Condition::Always,
                )
                .position(
                    [(DISPLAY_SIZE[0] * SCALE) as f32, 0.0],
                    imgui::Condition::Always,
                )
                .flags(flags)
                .build(&ui, || {
                    ui.text(im_str!("Main Registers: "));
                    ui.separator();
                    for i in 0..chip8_state.v.len() {
                        ui.text(im_str!("{:02}: {:02X?}", i, chip8_state.v[i]));
                    }
                    ui.separator();
                    ui.text(im_str!("Other Registers: "));
                    ui.separator();
                    ui.text(im_str!("i: {:02X?}", chip8_state.i));
                    ui.text(im_str!("stack pointer: {:02X?}", chip8_state.stack_pointer));
                    ui.text(im_str!("delay timer: {:02X?}", chip8_state.delay_timer));
                    ui.text(im_str!("sound timer: {:02X?}", chip8_state.sound_timer));
                    ui.text(im_str!(
                        "program counter: {:02X?}",
                        chip8_state.program_counter
                    ));
                });

            let memory_table_window_size = [
                DEBUG_EXTRA_DISPLAY_SIZE[0] + (DISPLAY_SIZE[0] * SCALE) as f32,
                DEBUG_EXTRA_DISPLAY_SIZE[1],
            ];
            Window::new(im_str!("Memory Table"))
                .size(
                    [memory_table_window_size[0], memory_table_window_size[1]],
                    imgui::Condition::Always,
                )
                .position(
                    [0.0, (DISPLAY_SIZE[1] * SCALE) as f32],
                    imgui::Condition::Always,
                )
                .flags(flags)
                .build(&ui, || {
                    let col_count = memory_table_window_size[0] as usize / 20;
                    let table_count = chip8_state.memory.len() / col_count;

                    for i in 0..table_count {
                        ui.text(im_str!("{:02X?}", chip8_state.memory[col_count * i]));
                        for j in 0..col_count {
                            ui.same_line(0.0);
                            ui.text(im_str!("{:02X?}", chip8_state.memory[col_count * i + j]));
                        }
                    }
                });
        }

        // Render
        let (factory, _, encoder, _, render_target) = graphics::gfx_objects(ctx);
        let draw_data = ui.render();
        self.renderer
            .render(
                &mut *factory,
                encoder,
                &mut RenderTargetView::new(render_target.clone()),
                draw_data,
            )
            .unwrap();
    }

    fn update_mouse(&mut self) {
        self.imgui.io_mut().mouse_pos =
            [self.mouse_state.pos.0 as f32, self.mouse_state.pos.1 as f32];

        self.imgui.io_mut().mouse_down = [
            self.mouse_state.pressed.0,
            self.mouse_state.pressed.1,
            self.mouse_state.pressed.2,
            false,
            false,
        ];

        self.imgui.io_mut().mouse_wheel = self.mouse_state.wheel;
        self.mouse_state.wheel = 0.0;

        self.imgui.io_mut().mouse_wheel_h = self.mouse_state.wheel_h;
        self.mouse_state.wheel_h = 0.0;
    }

    pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
        self.mouse_state.pos = (x as i32, y as i32);
    }

    pub fn update_mouse_down(&mut self, pressed: (bool, bool, bool)) {
        self.mouse_state.pressed = pressed;
    }

    pub fn update_key_down(&mut self, key: KeyCode, mods: KeyMods) {
        self.imgui.io_mut().key_shift = mods.contains(KeyMods::SHIFT);
        self.imgui.io_mut().key_ctrl = mods.contains(KeyMods::CTRL);
        self.imgui.io_mut().key_alt = mods.contains(KeyMods::ALT);
        self.imgui.io_mut().keys_down[key as usize] = true;
    }

    pub fn update_key_up(&mut self, key: KeyCode, mods: KeyMods) {
        if mods.contains(KeyMods::SHIFT) {
            self.imgui.io_mut().key_shift = false;
        }
        if mods.contains(KeyMods::CTRL) {
            self.imgui.io_mut().key_ctrl = false;
        }
        if mods.contains(KeyMods::ALT) {
            self.imgui.io_mut().key_alt = false;
        }
        self.imgui.io_mut().keys_down[key as usize] = false;
    }

    pub fn update_text(&mut self, val: char) {
        self.imgui.io_mut().add_input_character(val);
    }

    pub fn update_scroll(&mut self, x: f32, y: f32) {
        self.mouse_state.wheel += y;
        self.mouse_state.wheel_h += x;
    }
}
