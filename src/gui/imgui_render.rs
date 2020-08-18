use imgui::*;
use imgui_gfx_renderer::*;

use super::super::chip8_state::*;

pub fn render_gui(ui: &Ui, chip8_state: &Chip8State) {
    let flags = imgui::WindowFlags::NO_RESIZE
        | imgui::WindowFlags::NO_MOVE
        | imgui::WindowFlags::NO_COLLAPSE;

    render_register_info_window(ui, chip8_state, flags);
    render_memory_table(ui, chip8_state, flags);
}

fn render_register_info_window(ui: &Ui, chip8_state: &Chip8State, flags: WindowFlags) {
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
                ui.text(im_str!("{:02X?}: {:02X?}", i, chip8_state.v[i]));
            }
            ui.separator();
            ui.text(im_str!("Other Registers: "));
            ui.separator();
            ui.text(im_str!("i: {:02X?}", chip8_state.i));
            ui.text(im_str!("stack pointer: {:02X?}", chip8_state.stack_pointer));
            ui.text(im_str!("delay timer: {:02X?}", chip8_state.delay_timer));
            ui.text(im_str!("sound timer: {:02X?}", chip8_state.sound_timer));
            ui.text(im_str!(
                "program counter: {:03X?}",
                chip8_state.program_counter
            ));
        });
}

fn render_memory_table(ui: &Ui, chip8_state: &Chip8State, flags: WindowFlags) {
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
            let col_count = memory_table_window_size[0] as usize / 22;
            let table_count = chip8_state.memory.len() / col_count;

            for i in 0..table_count {
                for j in 0..col_count {
                    let index = (col_count * i + j) as u16;
                    let text = im_str!("{:02X?}", chip8_state.memory[col_count * i + j]);

                    if index == chip8_state.program_counter
                        || index == chip8_state.program_counter + 1
                    {
                        ui.text_colored([1.0, 0.0, 0.5, 1.0], text);
                    } else {
                        ui.text(text);
                    }
                    ui.same_line(0.0);
                }
                ui.dummy([0.0, 0.0]);
            }
        });
}
