use crate::{bytecode::BReader, lprimative::LPrimitive};
use bytes::Buf;

use super::{bsrc_lines::BSrcLine, blist::BList, binstruction::BInstruction, blocal::BLocal, bupvalue::BUpvalue, breader::BReadable};

#[derive(Debug)]
pub struct BProto {
    source_name: String,
    line_defined: i64,
    last_line_defined: i64,
    num_upvalues: u8,
    num_params: u8,
    vararg_flag: u8,
    max_stack: u8,
    instructions: BList<BInstruction>,
    constants: BList<LPrimitive>,
    protos: BList<BProto>,
    src_lines: BList<BSrcLine>,
    locals: BList<BLocal>,
    upvalues: BList<BUpvalue>
    
}
impl BReadable for BProto {
    fn read(reader: &mut BReader) -> Self {
        let source_name = reader.get_string();
        let line_defined = reader.get_int();
        let last_line_defined = reader.get_int();
        let num_upvalues = reader.inner.get_u8();
        let num_params = reader.inner.get_u8();
        let vararg_flag = reader.inner.get_u8();
        let max_stack = reader.inner.get_u8();
        
        let instructions = BList::read(reader);
        let constants = BList::read(reader);
        let protos = BList::read(reader);
        let src_lines = BList::read(reader);
        let locals = BList::read(reader);
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
            upvalues
        }
    }
}
