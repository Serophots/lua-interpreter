use super::breader::{BReader, BReadable};

/// Instructions, constants, protos, locals, upvalues
#[derive(Debug)]
pub struct BList<T: BReadable> {
    list: Vec<T>,
}

impl<T> BList<T>
where T: BReadable
{
    pub fn read(reader: &mut BReader) -> Self {
        let size = reader.get_int();
        let mut list = Vec::with_capacity(size as usize);

        for _ in 0..size {
            list.push(T::read(reader))
        }

        Self {
            list,
        }
    }
}