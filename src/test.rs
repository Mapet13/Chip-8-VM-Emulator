use super::*;

fn get_vm() -> Chip8State {
    Chip8State {
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
    }
}

#[test]
fn test_00E0() {
    // 0x00E0 - Clear the screen

    let opcode = 0x00E0;
    let mut vm = get_vm();

    vm.display_data = [true; DISPLAY_SIZE[0] * DISPLAY_SIZE[1]];

    vm.execute_instruction(decode_opcode(opcode), opcode);

    for i in 0..vm.display_data.len() {
        assert_eq!(vm.display_data[i], false);
    }
}

#[test]
fn test_00EE() {
    // 0x00EE - Return from a subroutine

    let opcode = 0x00EE;
    let mut vm = get_vm();

    vm.stack[0x0] = 0x200;
    vm.stack_pointer = 1;

    vm.execute_instruction(decode_opcode(opcode), opcode);

    assert_eq!(vm.program_counter, 0x200);
    assert_eq!(vm.stack_pointer, 0);
}

#[test]
fn test_1NNN() {
    // 0x1NNN - Jump to address NNN

    let opcode = 0x1234;
    let mut vm = get_vm();

    vm.execute_instruction(decode_opcode(opcode), opcode);

    assert_eq!(vm.program_counter + 2, 0x0234); // add 2 to jump result because in normal execution after jump CPU will increase pc by 2
}

#[test]
fn test_2NNN() {
    // 0x2NNN - Execute subroutine starting at address NNN

    let opcode = 0x2345;
    let mut vm = get_vm();

    let old_pc_value = vm.program_counter;

    vm.execute_instruction(decode_opcode(opcode), opcode);

    assert_eq!(vm.program_counter + 2, 0x0345); // add 2 to jump result because in normal execution after jump CPU will increase pc by 2
    assert_eq!(vm.stack_pointer, 1);
    assert_eq!(vm.stack[0x0], old_pc_value);
}

#[test]
fn test_3XNN() {
    // 0x3XNN - Skip the following instruction if the value of register VX equals NN

    let opcode = 0x3456;
    let mut vm = get_vm();

    //not equal
    vm.program_counter = 0x0;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.program_counter, 0x0);

    //equal
    vm.program_counter = 0x0;
    vm.v[0x4] = 0x56;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.program_counter, 0x2);
}

#[test]
fn test_4XNN() {
    // 0x4XNN - Skip the following instruction if the value of register VX is not equal to NN

    let opcode = 0x4567;
    let mut vm = get_vm();

    //not equal
    vm.program_counter = 0x0;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.program_counter, 0x2);

    //equal
    vm.program_counter = 0x0;
    vm.v[0x5] = 0x67;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.program_counter, 0x0);
}

#[test]
fn test_5XY0() {
    // 0x5XY0 - Skip the following instruction if the value of register VX is equal to the value of register VY

    let opcode = 0x5670;
    let mut vm = get_vm();

    //not equal
    vm.v[0x6] = 0x0;
    vm.v[0x7] = 0x1;
    vm.program_counter = 0x0;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.program_counter, 0x0);
    //equal
    vm.v[0x6] = 0x1;
    vm.v[0x7] = 0x1;
    vm.program_counter = 0x0;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.program_counter, 0x2);
}

#[test]
fn test_6XNN() {
    // 0x6XNN - Store number NN in register VX

    let opcode = 0x6789;
    let mut vm = get_vm();

    vm.v[0x7] = 0x0;
    vm.execute_instruction(decode_opcode(opcode), opcode);

    assert_eq!(vm.v[0x7], 0x89);
}

#[test]
fn test_7XNN() {
    // 0x7XNN - Add the value NN to register VX

    let opcode = 0x789A;
    let mut vm = get_vm();

    vm.v[0x8] = 0x11;
    vm.execute_instruction(decode_opcode(opcode), opcode);

    assert_eq!(vm.v[0x8], 0xAB);
}

#[test]
fn test_8XY0() {
    // 0x8XY0 - Store the value of register VY in register VX

    let opcode = 0x89A0;
    let mut vm = get_vm();

    vm.v[0x9] = 0x99;
    vm.v[0xA] = 0xAA;
    vm.execute_instruction(decode_opcode(opcode), opcode);

    assert_eq!(vm.v[0x9], vm.v[0xA]);
    assert_eq!(vm.v[0x9], 0xAA);
}

#[test]
fn test_8XY1() {
    // 0x8XY1 - Set VX to VX OR VY

    let opcode = 0x89A1;
    let mut vm = get_vm();

    vm.v[0x9] = 0x99;
    vm.v[0xA] = 0xAA;
    vm.execute_instruction(decode_opcode(opcode), opcode);

    assert_eq!(vm.v[0x9], 0x99 | 0xAA);
}

#[test]
fn test_8XY2() {
    // 0x8XY2 - Set VX to VX AND VY

    let opcode = 0x89A2;
    let mut vm = get_vm();

    vm.v[0x9] = 0x99;
    vm.v[0xA] = 0xAA;
    vm.execute_instruction(decode_opcode(opcode), opcode);

    assert_eq!(vm.v[0x9], 0x99 & 0xAA);
}

#[test]
fn test_8XY3() {
    // 0x8XY3 - Set VX to VX XOR VY

    let opcode = 0x89A3;
    let mut vm = get_vm();

    vm.v[0x9] = 0x99;
    vm.v[0xA] = 0xAA;

    vm.execute_instruction(decode_opcode(opcode), opcode);

    assert_eq!(vm.v[0x9], 0xAA ^ 0x99);
}

#[test]
fn test_8XY4() {
    // 0x8XY4 - Add the value of register VY to register VX
    //        Set VF to 01 if a carry occurs
    //        Set VF to 00 if a carry does not occur

    let opcode = 0x89A4;
    let mut vm = get_vm();

    // with borrow
    vm.v[0x9] = 0x99;
    vm.v[0xA] = 0xAA;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.v[0x9], 0x43);
    assert_eq!(vm.v[0xF], 0x1);

    // without borrow
    vm.v[0x9] = 0x11;
    vm.v[0xA] = 0x22;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.v[0x9], 0x33);
    assert_eq!(vm.v[0xF], 0x0);
}

#[test]
fn test_8XY5() {
    // 0x8XY5 - Subtract the value of register VY from register VX
    //          Set VF to 00 if a borrow occurs
    //          Set VF to 01 if a borrow does not occur

    let opcode = 0x89A5;
    let mut vm = get_vm();

    // without borrow
    vm.v[0x9] = 0xFF;
    vm.v[0xA] = 0x01;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.v[0x9], 0xFE);
    assert_eq!(vm.v[0xF], 0x1);

    // with borrow
    vm.v[0x9] = 0x01;
    vm.v[0xA] = 0x02;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.v[0x9], 0xFF);
    assert_eq!(vm.v[0xF], 0x0);
}

#[test]
fn test_8XY6() {
    // 0x8XY6 - Store the value of register VY shifted right one bit in register VX
    //        Set register VF to the least significant bit prior to the shift
    //        VY is unchange

    let opcode = 0x89A6;
    let mut vm = get_vm();

    // the least-significant bit of Vx is 1
    vm.v[0xA] = 0xFF;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.v[0x9], 0x7F);
    assert_eq!(vm.v[0xF], 0x1);

    // the least-significant bit of Vx is 0
    vm.v[0xA] = 0xFE;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.v[0x9], 0x7F);
    assert_eq!(vm.v[0xF], 0x0);
}

#[test]
fn test_8XY7() {
    // 0x8XY7 - Set register VX to the value of VY minus VX
    //        Set VF to 00 if a borrow occurs
    //        Set VF to 01 if a borrow does not occur

    let opcode = 0x89A7;
    let mut vm = get_vm();

    // without borrow
    vm.v[0x9] = 0x02;
    vm.v[0xA] = 0x08;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.v[0x9], 0x06);
    assert_eq!(vm.v[0xF], 0x1);

    // with borrow
    vm.v[0x9] = 0x04;
    vm.v[0xA] = 0x02;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.v[0x9], 0xFE);
    assert_eq!(vm.v[0xF], 0x0);
}

#[test]
fn test_8XYE() {
    // 0x8XYE - Store the value of register VY shifted left one bit in register VX
    //        Set register VF to the most significant bit prior to the shift
    //        VY is unchanged

    let opcode = 0x89AE;
    let mut vm = get_vm();

    // the most-significant bit of Vx is 0
    vm.v[0xA] = 0x11;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.v[0xF], 0x00);
    assert_eq!(vm.v[0xA], 0x11);
    assert_eq!(vm.v[0x9], 0x22);

    // the most-significant bit of Vx is 1
    vm.v[0xA] = 0x81;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.v[0xA], 0x81);
    assert_eq!(vm.v[0x9], 0x02);
    assert_eq!(vm.v[0xF], 0x01);
}

#[test]
fn test_9XY0() {
    // 0x9XY0 - Skip the following instruction if the value of register VX is not equal to the value of register VY

    let opcode = 0x9AB0;
    let mut vm = get_vm();

    //equal
    vm.v[0xA] = 0x1;
    vm.v[0xB] = 0x1;
    vm.program_counter = 0x0;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.program_counter, 0x0);
    //not equal
    vm.v[0xA] = 0x0;
    vm.v[0xB] = 0x1;
    vm.program_counter = 0x0;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.program_counter, 0x2);
}

#[test]
fn test_ANNN() {
    // 0xANNN - Store memory address NNN in register I

    let opcode = 0xABCD;
    let mut vm = get_vm();

    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.i, 0xBCD);
}

#[test]
fn test_BNNN() {
    // 0xBNNN - Jump to address NNN + V0

    let opcode = 0xBCDE;
    let mut vm = get_vm();
    vm.v[0x0] = 0x04;

    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.program_counter + 2, 0xCDE + 0x04); // add 2 to jump result because in normal execution after jump CPU will increase pc by 2
}

#[test]
fn test_CXNN() {
    // 0xCXNN - Set VX to a random number with a mask of NN

    let opcode = 0xCD00;
    let mut vm = get_vm();
    vm.v[0xD] = 0xFF;

    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.v[0xD], 0x00);
}

fn load_test_sprite(memory: &mut [u8; MEMORY_SIZE]) {
    memory[0xA] = 0xF0;
    memory[0xB] = 0x90;
    memory[0xC] = 0xF0;
    memory[0xD] = 0x90;
    memory[0xE] = 0xF0;
}

fn assert_sprite_drawing(display_data: &[bool; DISPLAY_SIZE[0] * DISPLAY_SIZE[1]]) {
    let assert_pixel = |x, y, expected: bool| {
        assert_eq!(
            display_data[y * DISPLAY_SIZE[0] + x],
            expected,
            "pixel [{}, {}] should be {}",
            x,
            y,
            expected
        );
    };
    assert_pixel(0, 0, true);
    assert_pixel(1, 0, true);
    assert_pixel(2, 0, true);
    assert_pixel(3, 0, true);
    assert_pixel(0, 1, true);
    assert_pixel(1, 1, false);
    assert_pixel(2, 1, false);
    assert_pixel(3, 1, true);
    assert_pixel(0, 2, true);
    assert_pixel(1, 2, true);
    assert_pixel(2, 2, true);
    assert_pixel(3, 2, true);
    assert_pixel(0, 3, true);
    assert_pixel(1, 3, false);
    assert_pixel(2, 3, false);
    assert_pixel(3, 3, true);
    assert_pixel(0, 4, true);
    assert_pixel(1, 4, true);
    assert_pixel(2, 4, true);
    assert_pixel(3, 4, true);
}

fn assert_sprite_ereasing(display_data: &[bool; DISPLAY_SIZE[0] * DISPLAY_SIZE[1]]) {
    let assert_pixel = |x, y, expected: bool| {
        assert_eq!(
            display_data[y * DISPLAY_SIZE[0] + x],
            expected,
            "pixel [{}, {}] should be {}",
            x,
            y,
            expected
        );
    };
    assert_pixel(0, 0, false);
    assert_pixel(1, 0, false);
    assert_pixel(2, 0, false);
    assert_pixel(3, 0, false);
    assert_pixel(0, 1, false);
    assert_pixel(1, 1, false);
    assert_pixel(2, 1, false);
    assert_pixel(3, 1, false);
    assert_pixel(0, 2, false);
    assert_pixel(1, 2, false);
    assert_pixel(2, 2, false);
    assert_pixel(3, 2, false);
    assert_pixel(0, 3, false);
    assert_pixel(1, 3, false);
    assert_pixel(2, 3, false);
    assert_pixel(3, 3, false);
    assert_pixel(0, 4, false);
    assert_pixel(1, 4, false);
    assert_pixel(2, 4, false);
    assert_pixel(3, 4, false);
}

#[test]
fn test_DXYN() {
    // 0xDXYN - Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
    //          Set VF to 01 if any set pixels are changed to unset, and 00 otherwise

    let opcode = 0xD005;
    let mut vm = get_vm();

    load_test_sprite(&mut vm.memory);
    vm.i = 0xA;
    vm.execute_instruction(decode_opcode(opcode), opcode);

    assert_sprite_drawing(&vm.display_data);
    assert_eq!(vm.v[0xF], 0x00);

    vm.execute_instruction(decode_opcode(opcode), opcode);

    assert_sprite_ereasing(&vm.display_data);
    assert_eq!(vm.v[0xF], 0x01);
}

#[test]
fn test_EX9E() {
    // 0xEX9E - skip the following instruction if the key corresponding to the hex value currently stored in register VX is pressed

    let opcode = 0xEE9E;
    let mut vm = get_vm();

    vm.v[0xE] = 0x01;
    //is pressed
    vm.program_counter = 0x00;
    vm.chip8_key = Some(0x01);
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.program_counter, 0x02);

    //is not pressed
    vm.chip8_key = Some(0x02);
    vm.program_counter = 0x00;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.program_counter, 0x00);
}

#[test]
fn test_EXA1() {
    // 0xEXA1 - skip the following instruction if the key corresponding to the hex value currently stored in register VX is not pressed

    let opcode = 0xEEA1;
    let mut vm = get_vm();

    vm.v[0xE] = 0x01;
    //is pressed
    vm.program_counter = 0x00;
    vm.chip8_key = Some(0x01);
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.program_counter, 0x00);

    //is not pressed
    vm.chip8_key = Some(0x02);
    vm.program_counter = 0x00;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.program_counter, 0x02);
}

#[test]
fn test_FX07() {
    // 0xFX07 - Store the current value of the delay timer in register VX

    let opcode = 0xF007;
    let mut vm = get_vm();

    vm.delay_timer = 0x32;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.v[0x0], vm.delay_timer);
}

#[test]
fn test_FX0A() {
    // 0xFX0A - skip the following instruction if the key corresponding to the hex value currently stored in register VX is not pressed

    let opcode = 0xF00A;
    let mut vm = get_vm();

    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.waiting_for_key_press, true);
}

#[test]
fn test_FX15() {
    // 0xFX15 - Set the delay timer to the value of register VX

    let opcode = 0xF015;
    let mut vm = get_vm();

    vm.delay_timer = 0x22;
    vm.v[0x0] = 0x33;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.delay_timer, vm.v[0x0]);
    assert_eq!(vm.delay_timer, 0x33);
}

#[test]
fn test_FX18() {
    // 0xEXA1 - Set the sound timer to the value of register VX

    let opcode = 0xF018;
    let mut vm = get_vm();

    vm.sound_timer = 0x22;
    vm.v[0x0] = 0x33;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.sound_timer, vm.v[0x0]);
    assert_eq!(vm.sound_timer, 0x33);
}

#[test]
fn test_FX1E() {
    // 0xEX1E - sAdd the value stored in register VX to register I

    let opcode = 0xF01E;
    let mut vm = get_vm();
    vm.i = 0x22;
    vm.v[0x0] = 0x33;
    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.i, 0x55);
}

#[test]
fn test_FX29() {
    // 0xFX29 - Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX

    let opcode = 0xF029;
    let mut vm = get_vm();

    vm.v[0x0] = 0x10;
    vm.execute_instruction(decode_opcode(opcode), opcode);

    assert_eq!(vm.i, 0x50);
}

#[test]
fn test_FX33() {
    // 0xFX33 - Store the binary-coded decimal equivalent of the value stored in register VX at addresses I, I + 1, and I + 2

    let opcode = 0xF033;
    let mut vm = get_vm();

    vm.v[0x0] = 123;
    vm.i = 0x0;

    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.memory[vm.i as usize], 1);
    assert_eq!(vm.memory[vm.i as usize + 1], 2);
    assert_eq!(vm.memory[vm.i as usize + 2], 3);
}

#[test]
fn test_FX55() {
    // 0xFX55 - Store the values of registers V0 to VX inclusive in memory starting at address I
    //          I is set to I + X + 1 after operation

    let opcode = 0xF455;
    let mut vm = get_vm();

    vm.i = 0x0;
    vm.v[0x0] = 0x0;
    vm.v[0x1] = 0x1;
    vm.v[0x2] = 0x2;
    vm.v[0x3] = 0x3;
    vm.v[0x4] = 0x4;

    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.memory[0x0], 0x00);
    assert_eq!(vm.memory[0x1], 0x01);
    assert_eq!(vm.memory[0x2], 0x02);
    assert_eq!(vm.memory[0x3], 0x03);
    assert_eq!(vm.memory[0x4], 0x04);
    assert_eq!(vm.i, 0x05);
}

#[test]
fn test_FX65() {
    // 0xFX65 - Fill registers V0 to VX inclusive with the values stored in memory starting at address I

    let opcode = 0xF465;
    let mut vm = get_vm();

    vm.i = 0x0;
    vm.memory[0x0] = 0x0;
    vm.memory[0x1] = 0x1;
    vm.memory[0x2] = 0x2;
    vm.memory[0x3] = 0x3;
    vm.memory[0x4] = 0x4;

    vm.execute_instruction(decode_opcode(opcode), opcode);
    assert_eq!(vm.v[0x0], 0x00);
    assert_eq!(vm.v[0x1], 0x01);
    assert_eq!(vm.v[0x2], 0x02);
    assert_eq!(vm.v[0x3], 0x03);
    assert_eq!(vm.v[0x4], 0x04);
    assert_eq!(vm.i, 0x05);
}
