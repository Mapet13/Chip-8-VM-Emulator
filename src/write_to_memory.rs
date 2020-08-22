use crate::utils::byte_copy;
use crate::fonts_sprites::FONTS_SPRITES;
use crate::chip8_state::CHIP8_RESERVED_MEMORY_SIZE;
use crate::chip8_state::MEMORY_SIZE;

pub fn write_rom_data_to_memory(memory: &mut [u8; MEMORY_SIZE], rom_data: &[u8]) {
    byte_copy(
        rom_data,
        &mut memory[CHIP8_RESERVED_MEMORY_SIZE..MEMORY_SIZE],
    );
}

pub fn write_font_data_to_memory(memory: &mut [u8; MEMORY_SIZE]) {
    for i in 0..FONTS_SPRITES.len() {
        for j in 0..FONTS_SPRITES[i].len() {
            memory[i * FONTS_SPRITES[i].len() + j] = FONTS_SPRITES[i][j];
        }
    }
}