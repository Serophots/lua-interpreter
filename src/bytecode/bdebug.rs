use super::breader::BReadable;

#[derive(Debug)]
pub struct BDebugLineInfo {
    pub line: i64,
}
impl BReadable for BDebugLineInfo {
    fn read(reader: &mut super::breader::BReader) -> Self {
        Self {
            line: reader.get_c_int(),
        }
    }
}

#[derive(Debug)]
pub struct BDebugLocal {
    local: String,
    scope_start: i64,
    scope_end: i64,
}
impl BReadable for BDebugLocal {
    fn read(reader: &mut super::breader::BReader) -> Self {
        Self {
            local: reader
                .get_string()
                .expect("Local had name string of length 0"),
            scope_start: reader.get_c_int(),
            scope_end: reader.get_c_int(),
        }
    }
}

#[derive(Debug)]
pub struct BDebugUpvalue {
    upvalue: String,
}
impl BReadable for BDebugUpvalue {
    fn read(reader: &mut super::breader::BReader) -> Self {
        Self {
            upvalue: reader
                .get_string()
                .expect("Upvalue had name string of length 0"),
        }
    }
}
