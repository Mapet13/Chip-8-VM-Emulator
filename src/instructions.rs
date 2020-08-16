pub enum InstructionSet {
    MachineLanguageSubroutine(u16),
    ClearScreen,
    ReturnFromSubroutine,
    JumpToAddress(u16),
    ExecuteSubroutine(u16),
    SkipFollowingIfRegisterIsEqualToValue(u8, u8),
    SkipFollowingIfRegisterIsNotEqualToValue(u8, u8),
    SkipFollowingIfRegisterIsEqualToOtherRegister(u8, u8),
    SkipFollowingIfRegisterIsNotEqualToOtherRegister(u8, u8),
    StoreInRegister(u8, u8),
    AddToRegister(u8, u8),
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

impl ToString for InstructionSet {
    fn to_string(&self) -> String {
        match self {
            InstructionSet::ClearScreen => {
                "Clearing the Screen".to_string()
            }
            InstructionSet::JumpToAddress(address) => {
                format!("Jump To Address [{:03X?}]", address)
            }
            InstructionSet::ExecuteSubroutine(_) => {
                "Execute Subroutine".to_string()
            }
            InstructionSet::StoreInRegister(index, value) => {
                format!(
                    "Store Value [{:02X?}] In Register [{:02X?}]",
                    value, index
                )
            }
            InstructionSet::AddToRegister(index, value) => {
                format!(
                    "Add Value [{:02X?}] To Register [{:02X?}]",
                     value, index
                )
            }
            InstructionSet::CopyRegisterValueToOtherRegister(x, y) => {
                format!(
                    "Copy Register [{:02X?}] Value To Other Register [{:02X?}]",
                     y, x
                )
            }
            InstructionSet::None => {
                    "Not Handled Opcode".to_string()
            }
            InstructionSet::SkipFollowingIfRegisterIsEqualToValue(register_index, value) => {
                format!(
                    "Skip Following If Register [{:02X?}] Is Equal To Value [{:02X?}]",
                     register_index, value,
                )
            }
            InstructionSet::SetVxToVxOrVy(x, y) => {
                format!(
                    "Set Vx To Vx [{:02X?}] Or Vy [{:02X?}]",
                     x, y
                )
            }
            InstructionSet::SetVxToVxAndVy(x, y) => {
                format!(
                    "Set Vx [{:02X?}] To Vx And Vy [{:02X?}]",
                     x, y
                )
            }
            InstructionSet::SetVxToVxXorVy(x, y) => {
                format!(
                    "Set Vx [{:02X?}] To Vx Xor Vy [{:02X?}]",
                     x, y
                )
            }
            InstructionSet::SkipFollowingIfRegisterIsNotEqualToValue(register_index, value) => {
                format!(
                    "Skip Following If Register [{:02X?}] Is Not Equal To Value [{:02X?}]",
                    register_index,
                    value,
                )
            }
            InstructionSet::SkipFollowingIfRegisterIsEqualToOtherRegister(x, y) => {
                format!(
                    "Skip Following If Register [{:02X?}] Is Equal To Other Register [{:02X?}]",
                    x,
                    y
                )
            }
            InstructionSet::AddValueOfRegisterVyToRegisterVx(x, y) => {
                format!(
                    "Add Value Of Register Vy [{:02X?}] To Register Vx [{:02X?}]",
                     y, x
                )
            }
            InstructionSet::MachineLanguageSubroutine(_) => {
                    "Machine Language Subroutine".to_string()
            }
            InstructionSet::StoreAddressInRegisterI(address) => {
                format!(
                    "Store Address [{:03X?}] In Register I",
                     address
                )
            }
            InstructionSet::JumpToAddressWithV0Offset(address) => {
                format!(
                    "Jump To Address [{:03X?}] mWith V0 Offset",
                     address
                )
            }
            InstructionSet::SetVxToRandomNumberWithAMaskOf(x, mask) => {
                format!(
                    "Set Vx [{:02X?}] To Random Number With A Mask Of [{:02X?}]",
                     x, mask
                )
            }
            InstructionSet::DrawSprite(x, y, sprite_data) => {
                format!("Draw a sprite at position VX [{:02X?}], VY [{:02X?}] with N [{:02X?}] bytes of sprite data starting at the address stored in I", x, y, sprite_data)
            }
            InstructionSet::ReturnFromSubroutine => {
                "Return From Subroutine".to_string()
            }
            InstructionSet::SkipFollowingIfRegisterIsNotEqualToOtherRegister(x, y) => {
                format!(
                    "Skip Following If Register [{:02X?}] Is Not Equal To Other Register  [{:02X?}]",
                    x,
                    y
                )
            }
            InstructionSet::SubtractValueOfRegisterVyFromRegisterVx(x, y) => {
                format!(
                    "Subtract Value Of Register Vy [{:02X?}] From Register Vx [{:02X?}]",
                     y, x,
                )
            }
            InstructionSet::StoreValueOfRegisterVyShiftedRightOneBitInVx(x, y) => {
                format!(
                    "Store Value Of Register Vy [{:02X?}] Shifted Right One Bit In Vx [{:02X?}]",  
                    y,
                    x,
                )
            }
            InstructionSet::SetVxToValueOfVyMinusVx(x, y) => {
                format!(
                    "Set Vx [{:02X?}] To Value Of Vy [{:02X?}] Minus Vx",
                     x, y
                )
            }
            InstructionSet::StoreValueOfRegisterVyShiftedLeftOneBitInVx(x, y) => {
                format!(
                    "Store Value Of Register Vy [{:02X?}] Shifted Left One Bit In Vx [{:02X?}]",
                    y,
                    x,
                )
            }
            InstructionSet::StoreDelayTimerInRegisterVx(x) => {
                format!(
                    "Store Delay Timer In Register Vx [{:02X?}]",
                     x
                )
            }
            InstructionSet::WaitForAKeyPress(x) => {
                format!(
                    "Wait For A Key Press And Store The Result In Register VX [{:02X?}]",
                     x
                )
            }
            InstructionSet::SkipFollowingInstructionIfKeyCorrespondingToVxIsPressed(x) => {
                format!(
                    "Skip Following Instruction If Key Corresponding To Vx [{:02X?}] Is Pressed",
                    x
                )
            }
            InstructionSet::SkipFollowingInstructionIfKeyCorrespondingToVxIsNotPressed(x) => {
                format!(
                    "Skip Following Instruction If Key Corresponding To Vx [{:02X?}] Is Not Pressed",
                    x
                )
            }
            InstructionSet::SetDelayTimerToVx(x) => {
                format!("Set Delay Timer To Vx [{:02X?}]", x)
            }
            InstructionSet::SetSoundTimerToVx(x) => {
                format!("Set Sound Timer To Vx [{:02X?}]", x)
            }
            InstructionSet::AddVxToRegisterI(x) => {
                format!("Add Vx [{:02X?}] To Register I", x)
            }
            InstructionSet::SetIToTheMemoryAddressOfSpriteCorrespondingToVx(x) => {
                format!(
                    "Set I To The Memory Address Of Sprite Corresponding To Vx [{:02X?}]",
                     x
                )
            }
            InstructionSet::StoreTheBinaryCodedDecimalEquivalentOfVx(x) => {
                format!(
                    "Store The Binary-Coded Decimal Equivalent Of Vx [{:02X?}]",
                     x
                )
            }
            InstructionSet::StoreValuesOfV0ToVxInclusiveInMemoryStartingAtAddressI(x) => {
                format!(
                    "Store Values Of V0 To Vx [{:02X?}] Inclusive In Memory Starting At Address I",
                     x
                )
            }
            InstructionSet::FillRegistersV0ToVxInclusiveWithMemoryStartingAtAddressI(x) => {
                format!(
                    "Fill Registers V0 To Vx [{:02X?}] Inclusive With Memory Starting At Address I",
                     x
                )
            }
        }
    }
}

pub fn decode_opcode(opcode: u16) -> InstructionSet {
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
