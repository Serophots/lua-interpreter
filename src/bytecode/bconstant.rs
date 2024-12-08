use bytes::Buf;

use super::breader::{BReadable, BReader};
use crate::lprimative::LPrimitive;

impl BReadable for LPrimitive {
    fn read(reader: &mut BReader) -> LPrimitive {
        let indicator = reader.inner.get_u8();
        match indicator {
            0 => Self::NIL,
            1 => Self::BOOL(reader.inner.get_u8() != 0),
            3 => Self::NUMBER(reader.get_lua_number()),
            4 => Self::STRING(reader.get_string().expect("String constant had length 0")),
            n => panic!(
                "Attempted to read constant of unrecognised type indicator {}",
                n
            ),
        }
    }
}
