use bytes::Buf;

use crate::lprimative::LPrimitive;
use super::breader::{BReadable, BReader};

impl BReadable for LPrimitive {
    fn read(reader: &mut BReader) -> LPrimitive {
        let indicator = reader.inner.get_u8();
        match indicator {
            0 => Self::NIL,
            1 => {
                Self::BOOL(reader.inner.get_u8() != 0)
            },
            3 => {
                Self::NUMBER(reader.get_number())
            },
            4 => {
                Self::STRING(reader.get_string())
            },
            n => panic!("Attempted to read constant of unrecognised type indicator {}", n),
        }
    }
}