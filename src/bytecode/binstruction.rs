use crate::bytecode::bopmode::BOpmode;

use super::breader::BReadable;

const OPCODE_MASK: u32 = 0b00000000000000000000000000111111;
const REG_AA_MASK: u32 = 0b00000000000000000011111111000000;
const REG_CC_MASK: u32 = 0b00000000011111111100000000000000;
const REG_BB_MASK: u32 = 0b11111111100000000000000000000000;
const REG_BX_MASK: u32 = 0b11111111111111111100000000000000;

#[derive(Debug)]
pub enum BInstruction {
    ABC{opcode: u8, a: u8, b: u16, c: u16},
    ABx{opcode: u8, a: u8, b: u32},
    AsBx{opcode: u8, a: u8, b: i32},
}
impl BReadable for BInstruction {
    fn read(reader: &mut super::breader::BReader) -> Self {
        let instruction = reader.get_u32_he();
        
        //Read opcode
        let opcode = (instruction & OPCODE_MASK) as u8;
        let opmode = BOpmode::from_opcode(opcode);

        //Read A reg
        let a = (((instruction & REG_AA_MASK) << 18) >> 24) as u8; 

        match opmode {
            BOpmode::ABC => {
                let b = ((instruction & REG_BB_MASK) >> 23) as u16;
                let c = (((instruction & REG_CC_MASK) << 9) >> 23) as u16;
                
                Self::ABC { opcode, a, b, c }
            },
            BOpmode::ABx => {
                let b = ((instruction & REG_BX_MASK) >> 14) as u32;
                Self::ABx { opcode, a, b }
            },
            BOpmode::AsBx => {
                //todo signed -> equality or clever bitshift?
                let b = ((instruction & REG_BX_MASK) >> 14) as u32;
                Self::AsBx { opcode, a, b: b as i32 }
            },
        }
    }
}