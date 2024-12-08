use crate::{bytecode::BReader, lprimative::LPrimitive};

use super::{
    binstruction::BInstruction, blines::BSrcLine, blist::BList, blocal::BLocal, breader::BReadable,
    bupvalue::BUpvalue,
};

/// https://www.lua.org/source/5.3/ldump.c.html#DumpFunction
#[derive(Debug)]
pub struct BProto {
    //Source name is defined only of the top level proto and therefore is read in the headers
    pub(crate) source_name: Option<String>, //DumpString
    pub(crate) line_defined: i64,           //DumpInt
    pub(crate) last_line_defined: i64,      //DumpInt
    pub(crate) num_params: u8,              //DumpByte
    pub(crate) vararg_flag: u8,             //DumpByte
    pub(crate) max_stack: u8,               //DumpByte

    pub(crate) instructions: BList<BInstruction>,
    pub(crate) constants: BList<LPrimitive>,
    pub(crate) upvalues: BList<BUpvalue>, //optional
    pub(crate) protos: BList<BProto>,

    pub(crate) src_lines: BList<BSrcLine>, //optional
    pub(crate) locals: BList<BLocal>,      //optional
}
impl BReadable for BProto {
    fn read(reader: &mut BReader) -> Self {
        println!("Reading a proto {}", reader.remaining());
        let source_name = reader.get_string(); //For the top level proto this is defined. For the others get_string() returns None because the string has length -1. So it must be called for all protos even if we know only the top proto has itt
        println!("src name {:?}", source_name);
        let line_defined = reader.get_c_int();
        let last_line_defined = reader.get_c_int();
        let num_params = reader.get_byte();
        let vararg_flag = reader.get_byte();
        let max_stack = reader.get_byte();

        println!("Reading instructions");
        let instructions = BList::read(reader);
        println!("Reading constants");
        let constants = BList::read(reader);
        println!("Reading upvalues");
        let upvalues = BList::read(reader);
        println!("Reading protos");
        let protos = BList::read(reader);

        println!("Reading src lines");
        let src_lines = BList::read(reader);
        println!("Reading locals");
        let locals = BList::read(reader);

        Self {
            source_name,
            line_defined,
            last_line_defined,
            num_params,
            vararg_flag,
            max_stack,

            instructions,
            constants,
            protos,
            src_lines,
            locals,
            upvalues,
        }
    }
}
