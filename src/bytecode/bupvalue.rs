use super::breader::BReadable;

#[derive(Debug)]
pub struct BUpvalue {
    name: String,
}
impl BReadable for BUpvalue {
    fn read(reader: &mut super::breader::BReader) -> Self {
        Self {
            name: reader.get_string().expect("Upvalue name had length 0"),
        }
    }
}
