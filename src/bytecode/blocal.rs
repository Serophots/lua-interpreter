use super::breader::BReadable;

#[derive(Debug)]
pub struct BLocal {
    local_name: String,
    scope_start: i64,
    scope_end: i64,
}
impl BReadable for BLocal {
    fn read(reader: &mut super::breader::BReader) -> Self {
        Self {
            local_name: reader
                .get_string()
                .expect("Local had name string of length 0"),
            scope_start: reader.get_int(),
            scope_end: reader.get_int(),
        }
    }
}
