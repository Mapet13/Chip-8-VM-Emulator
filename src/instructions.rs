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
            InstructionSet::SkipFollowingIfRegisterIsEqualToValue(register, value) => {
                format!(
                    "Skip Following If Register [{:02X?}] Is Equal To Value [{:02X?}]",
                     register, value,
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
            InstructionSet::SkipFollowingIfRegisterIsNotEqualToValue(register, value) => {
                format!(
                    "Skip Following If Register [{:02X?}] Is Not Equal To Value [{:02X?}]",
                    register,
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
                    "Jump To Address [{:03X?}] With V0 Offset",
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
    let address = opcode & 0x0FFF;
    let value = (opcode & 0x00FF) as u8;
    let x = ((opcode & 0x0F00) >> 8) as u8;
    let y = ((opcode & 0x00F0) >> 4) as u8;
    match opcode & 0xF000 {
        0x0000 => match opcode {
            0x00E0 => InstructionSet::ClearScreen,
            0x00EE => InstructionSet::ReturnFromSubroutine,
            _ => InstructionSet::MachineLanguageSubroutine(opcode),
        },
        0x1000 => InstructionSet::JumpToAddress(address),
        0x2000 => InstructionSet::ExecuteSubroutine(address),
        0x3000 => InstructionSet::SkipFollowingIfRegisterIsEqualToValue(x, value),
        0x4000 => InstructionSet::SkipFollowingIfRegisterIsNotEqualToValue(x, value),
        0x5000 => match opcode & 0xF00F {
            0x5000 => InstructionSet::SkipFollowingIfRegisterIsEqualToOtherRegister(x, y),
            _ => InstructionSet::None,
        },
        0x6000 => InstructionSet::StoreInRegister(x, value),
        0x7000 => InstructionSet::AddToRegister(x, value),
        0x8000 => match opcode & 0xF00F {
            0x8000 => InstructionSet::CopyRegisterValueToOtherRegister(x, y),
            0x8001 => InstructionSet::SetVxToVxOrVy(x, y),
            0x8002 => InstructionSet::SetVxToVxAndVy(x, y),
            0x8003 => InstructionSet::SetVxToVxXorVy(x, y),
            0x8004 => InstructionSet::AddValueOfRegisterVyToRegisterVx(x, y),
            0x8005 => InstructionSet::SubtractValueOfRegisterVyFromRegisterVx(x, y),
            0x8006 => InstructionSet::StoreValueOfRegisterVyShiftedRightOneBitInVx(x, y),
            0x8007 => InstructionSet::SetVxToValueOfVyMinusVx(x, y),
            0x800E => InstructionSet::StoreValueOfRegisterVyShiftedLeftOneBitInVx(x, y),
            _ => InstructionSet::None,
        },
        0x9000 => match opcode & 0xF00F {
            0x9000 => InstructionSet::SkipFollowingIfRegisterIsNotEqualToOtherRegister(x, y),
            _ => InstructionSet::None,
        },
        0xA000 => InstructionSet::StoreAddressInRegisterI(opcode & 0x0FFF),
        0xB000 => InstructionSet::JumpToAddressWithV0Offset(opcode & 0x0FFF),
        0xC000 => InstructionSet::SetVxToRandomNumberWithAMaskOf(x, value),
        0xD000 => {
            let sprite_data = (opcode & 0x000F) as u8;
            InstructionSet::DrawSprite(x, y, sprite_data)
        }
        0xE000 => match opcode & 0xF0FF {
            0xE09E => InstructionSet::SkipFollowingInstructionIfKeyCorrespondingToVxIsPressed(x),
            0xE0A1 => InstructionSet::SkipFollowingInstructionIfKeyCorrespondingToVxIsNotPressed(x),
            _ => InstructionSet::None,
        },
        0xF000 => match opcode & 0xF0FF {
            0xF007 => InstructionSet::StoreDelayTimerInRegisterVx(x),
            0xF00A => InstructionSet::WaitForAKeyPress(x),
            0xF015 => InstructionSet::SetDelayTimerToVx(x),
            0xF018 => InstructionSet::SetSoundTimerToVx(x),
            0xF01E => InstructionSet::AddVxToRegisterI(x),
            0xF029 => InstructionSet::SetIToTheMemoryAddressOfSpriteCorrespondingToVx(x),
            0xF033 => InstructionSet::StoreTheBinaryCodedDecimalEquivalentOfVx(x),
            0xF055 => InstructionSet::StoreValuesOfV0ToVxInclusiveInMemoryStartingAtAddressI(x),
            0xF065 => InstructionSet::FillRegistersV0ToVxInclusiveWithMemoryStartingAtAddressI(x),
            _ => InstructionSet::None,
        },
        _ => InstructionSet::None,
    }
}
