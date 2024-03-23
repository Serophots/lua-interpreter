use std::{
    env, fs,
    process::Command, io::Cursor,
};
use crate::bytecode::breader::BReadable;
use crate::bytecode::bproto::BProto;

use self::breader::BReader;

mod binstruction;
mod blist;
mod bproto;
mod breader;
mod bopmode;
mod bconstant;
mod bsrc_lines;
mod blocal;
mod bupvalue;

fn dump_bytecode() -> Result<Vec<u8>, std::io::Error> {
    let mut io_dir = env::current_dir().expect("failed to get current_dir");
    io_dir.push("io");

    Command::new("./io/lua5.1.exe")
        .args([r"dump.lua", r"input.lua", r"output.lua"])
        .current_dir(io_dir)
        .output()
        .expect("failed to dump bytecode");
    
    fs::read("io/output.lua")
}

pub fn decode_bytecode() -> Result<BProto, std::io::Error> {
    //Dump bytecode
    let bytecode = dump_bytecode()?;
    
    let mut reader = BReader::from_headers(Cursor::new(bytecode));
    let proto = BProto::read(&mut reader);
    
    Ok(proto)
}
