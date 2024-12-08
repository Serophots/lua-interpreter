use crate::bytecode::bproto::BProto;
use crate::bytecode::breader::BReadable;
use std::{env, fs, io::Cursor, process::Command};

use self::breader::BReader;

pub(crate) mod bconstant;
pub(crate) mod binstruction;
pub(crate) mod blines;
pub(crate) mod blist;
pub(crate) mod blocal;
pub(crate) mod bopcode;
pub(crate) mod bproto;
pub(crate) mod breader;
pub(crate) mod bupvalue;

fn dump_bytecode() -> Result<Vec<u8>, std::io::Error> {
    let mut io_dir = env::current_dir().expect("failed to get current_dir");
    io_dir.push("io");

    Command::new("./io/lua53.exe")
        .args([r"dump.lua", r"input.lua", r"output.lua"])
        .current_dir(io_dir)
        .output()
        .expect("failed to dump bytecode");

    fs::read("io/output.lua")
}

pub fn decode_bytecode() -> Result<Box<BProto>, std::io::Error> {
    //Dump bytecode
    let bytecode = dump_bytecode()?;

    let mut reader = BReader::from_headers(Cursor::new(bytecode));
    println!("read headers");
    let proto = Box::new(BProto::read(&mut reader));

    println!("read bytecode {:#?}", proto);

    Ok(proto)
}
