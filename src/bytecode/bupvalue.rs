use super::breader::BReadable;

#[derive(Debug)]
pub struct BUpvalue {
    stack_flag: u8,
    index: u8,
}
impl BReadable for BUpvalue {
    fn read(reader: &mut super::breader::BReader) -> Self {
        Self {
            stack_flag: reader.get_byte(),
            index: reader.get_byte(),
        }
    }
}
