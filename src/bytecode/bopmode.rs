use BOpmode::*;

#[derive(Copy, Clone)]
pub enum BOpmode {
    ABC,
    ABx,
    AsBx
}
const OPMODES: [BOpmode; 38] = [
    ABC, ABx, ABC, ABC, ABC, ABx, ABC, ABx, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, AsBx, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, AsBx, AsBx, ABC, ABC, ABC, ABx, ABC
];
impl BOpmode {
    pub fn from_opcode(opcode: u8) -> Self {
        match OPMODES.get(opcode as usize) {
            Some(x) => *x,
            None => panic!("Opcode {} not recognised", opcode)
        }
    }
}