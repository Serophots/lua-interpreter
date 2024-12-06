use BOpmode::*;

pub(crate) const OPMODES: [BOpmode; 38] = [
    ABC, ABx, ABC, ABC, ABC, ABx, ABC, ABx, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC,
    ABC, ABC, ABC, AsBx, ABC, ABC, ABC, ABC, ABC, ABC, ABC, ABC, AsBx, AsBx, ABC, ABC, ABC, ABx,
    ABC,
];
pub(crate) const OPNAMES: [&str; 38] = [
    "MOVE",
    "LOADK",
    "LOADBOOL",
    "LOADNIL",
    "GETUPVAL",
    "GETGLOBAL",
    "GETTABLE",
    "SETGLOBAL",
    "SETUPVAL",
    "SETTABLE",
    "NEWTABLE",
    "SELF",
    "ADD",
    "SUB",
    "MUL",
    "DIV",
    "MOD",
    "POW",
    "UNM",
    "NOT",
    "LEN",
    "CONCAT",
    "JMP",
    "EQ",
    "LT",
    "LE",
    "TEST",
    "TESTSET",
    "CALL",
    "TAILCALL",
    "RETURN",
    "FORLOOP",
    "FORPREP",
    "TFORLOOP",
    "SETLIST",
    "CLOSE",
    "CLOSURE",
    "VARARG",
];

#[derive(Copy, Clone)]
pub enum BOpmode {
    ABC,
    ABx,
    AsBx,
}

impl BOpmode {
    pub fn from_opcode(opcode: u8) -> Self {
        match OPMODES.get(opcode as usize) {
            Some(x) => *x,
            None => panic!("Opcode {} not recognised", opcode),
        }
    }
}
