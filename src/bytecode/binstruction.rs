use std::fmt;

use super::{
    bopcode::{Opmode, OPNAMES},
    breader::BReadable,
};

const OPCODE_MASK: u32 = 0b00000000000000000000000000111111;
const REG_AA_MASK: u32 = 0b00000000000000000011111111000000;
const REG_CC_MASK: u32 = 0b00000000011111111100000000000000;
const REG_BB_MASK: u32 = 0b11111111100000000000000000000000;
const REG_BX_MASK: u32 = 0b11111111111111111100000000000000;

#[derive(Copy, Clone)]
pub enum BInstruction {
    ABC {
        line: Option<i64>, //Optional debug line data
        opcode: u8,
        a: u8,
        b: u16,
        c: u16,
    },
    ABx {
        line: Option<i64>, //Optional debug line data
        opcode: u8,
        a: u8,
        b: u32,
    },
    AsBx {
        line: Option<i64>, //Optional debug line data
        opcode: u8,
        a: u8,
        b: i32,
    },
}
impl BReadable for BInstruction {
    fn read(reader: &mut super::breader::BReader) -> Self {
        let instruction = reader.get_u32();

        //Read opcode
        let opcode = (instruction & OPCODE_MASK) as u8;
        println!("opcode {}", opcode);
        let opmode = Opmode::from_opcode(opcode);

        //Read A reg
        let a = (((instruction & REG_AA_MASK) << 18) >> 24) as u8;

        match opmode {
            Opmode::ABC => {
                let b = ((instruction & REG_BB_MASK) >> 23) as u16;
                let c = (((instruction & REG_CC_MASK) << 9) >> 23) as u16;

                Self::ABC {
                    opcode,
                    a,
                    b,
                    c,
                    line: None,
                }
            }
            Opmode::ABx => {
                let b = ((instruction & REG_BX_MASK) >> 14) as u32;
                Self::ABx {
                    opcode,
                    a,
                    b,
                    line: None,
                }
            }
            Opmode::AsBx => {
                //todo signed -> equality or clever bitshift?
                let b = ((instruction & REG_BX_MASK) >> 14) as u32;
                Self::AsBx {
                    opcode,
                    a,
                    b: b as i32,
                    line: None,
                }
            }
        }
    }
}
impl fmt::Debug for BInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BInstruction::ABC {
                line,
                opcode,
                a,
                b,
                c,
            } => f
                .debug_struct("BInstruction")
                .field("op", &OPNAMES[*opcode as usize])
                .field("type", &"ABC")
                .field("a", &a)
                .field("b", &b)
                .field("c", &c)
                .field("line", line)
                .finish(),
            BInstruction::ABx { line, opcode, a, b } => f
                .debug_struct("BInstruction")
                .field("op", &OPNAMES[*opcode as usize])
                .field("type", &"ABx")
                .field("a", &a)
                .field("b", &b)
                .field("line", line)
                .finish(),
            BInstruction::AsBx { line, opcode, a, b } => f
                .debug_struct("BInstruction")
                .field("op", &OPNAMES[*opcode as usize])
                .field("type", &"AsBx")
                .field("a", &a)
                .field("b", &b)
                .field("line", line)
                .finish(),
        }
    }
}
