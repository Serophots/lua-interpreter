use super::breader::BReadable;

#[derive(Debug)]
pub struct BSrcLine {
    inner: i64,
}
impl BReadable for BSrcLine {
    fn read(reader: &mut super::breader::BReader) -> Self {
        Self {
            inner: reader.get_c_int(),
        }
    }
}
