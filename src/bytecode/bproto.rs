use crate::{bytecode::BReader, lprimative::LPrimitive};
use bytes::Buf;

use super::{
    binstruction::BInstruction, blines::BSrcLine, blist::BList, blocal::BLocal, breader::BReadable,
    bupvalue::BUpvalue,
};

#[derive(Debug)]
pub struct BProto {
    //Source name is defined only of the top level proto and therefore is read in the headers
    pub(crate) source_name: Option<String>,
    pub(crate) line_defined: i64,
    pub(crate) last_line_defined: i64,
    pub(crate) num_upvalues: u8,
    pub(crate) num_params: u8,
    pub(crate) vararg_flag: u8,
    pub(crate) max_stack: u8,
    pub(crate) instructions: BList<BInstruction>,
    pub(crate) constants: BList<LPrimitive>,
    pub(crate) protos: BList<BProto>,
    pub(crate) src_lines: BList<BSrcLine>, //optional
    pub(crate) locals: BList<BLocal>,      //optional
    pub(crate) upvalues: BList<BUpvalue>,  //optional
}
impl BReadable for BProto {
    fn read(reader: &mut BReader) -> Self {
        // println!("Reading a proto");
        let source_name = reader.get_string(); //For the top level proto this is defined. For the others get_string() returns None because the string has length -1. So it must be called for all protos even if we know only the top proto has itt
        let line_defined = reader.get_int();
        let last_line_defined = reader.get_int();
        let num_upvalues = reader.inner.get_u8();
        let num_params = reader.inner.get_u8();
        let vararg_flag = reader.inner.get_u8();
        let max_stack = reader.inner.get_u8();

        // println!("Reading instructions");
        let instructions = BList::read(reader);
        // println!("Reading constants");
        let constants = BList::read(reader);
        // println!("Reading protos");
        let protos = BList::read(reader);
        // println!("Reading src lines");
        let src_lines = BList::read(reader);
        // println!("Reading locals");
        let locals = BList::read(reader);
        // println!("Reading upvalues");
        let upvalues = BList::read(reader);

        Self {
            source_name,
            line_defined,
            last_line_defined,
            num_upvalues,
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
