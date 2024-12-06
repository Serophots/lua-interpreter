use super::breader::{BReadable, BReader};

/// Instructions, constants, protos, locals, upvalues
#[derive(Debug)]
pub struct BList<T: BReadable> {
    pub(crate) list: Vec<T>,
}

impl<T> BList<T>
where
    T: BReadable,
{
    pub fn read(reader: &mut BReader) -> Self {
        let size = reader.get_int();
        // println!(" - blist read size: {}", size);
        let mut list = Vec::with_capacity(size as usize);

        for _ in 0..size {
            list.push(T::read(reader))
        }

        Self { list }
    }
}
