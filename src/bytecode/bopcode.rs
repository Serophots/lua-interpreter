use Opmode::*;

// pub(crate) const OPMODES: [Opmode; 38] = [
//     ABC, ABx, ABC, ABC, ABC, ABx, ABC, ABx, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC,
//     ABC, ABC, ABC, AsBx, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, AsBx, AsBx, ABC, ABC, ABC, ABx,
//     ABx,
// ];
pub(crate) const OPMODES: [Opmode; 47] = [
    ABC, ABx, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC,
    ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, AsBx, ABC, ABC, ABC, ABC, ABC, ABC, ABC,
    ABC, AsBx, AsBx, ABC, AsBx, ABC, ABx, ABC, AsBx,
];
pub(crate) const OPNAMES: [&str; 47] = [
    "MOVE", "LOADK", "LOADKX", "LOADBOOL", "LOADNIL", "GETUPVAL", "GETTABUP", "GETTABLE",
    "SETTABUP", "SETUPVAL", "SETTABLE", "NEWTABLE", "SELF", "ADD", "SUB", "MUL", "MOD", "POW",
    "DIV", "IDIV", "BAND", "BOR", "BXOR", "SHL", "SHR", "UNM", "BNOT", "NOT", "LEN", "CONCAT",
    "JMP", "EQ", "LT", "LE", "TEST", "TESTSET", "CALL", "TAILCALL", "RETURN", "FORLOOP", "FORPREP",
    "TFORCALL", "TFORLOOP", "SETLIST", "CLOSURE", "VARARG", "EXTRAARG",
];

#[derive(Copy, Clone)]
pub enum Opmode {
    ABC,
    ABx,
    AsBx,
}

impl Opmode {
    pub fn from_opcode(opcode: u8) -> Self {
        match OPMODES.get(opcode as usize) {
            Some(x) => *x,
            None => panic!("Opcode {} not recognised", opcode),
        }
    }
}
