use std::io::Cursor;

use bytes::Buf;

///Bytecode reader which reads Integer, String bytecode primatives from headers
pub struct BReader {
    pub inner: Cursor<Vec<u8>>,

    //Headers
    endianness: u8,    //1 byte
    int_size: u8,      //1 byte
    t_size: u8,        //1 byte
    l_num_size: u8,    //1 byte
    integral_flag: u8, //1 byte
}
impl BReader {
    pub fn from_headers(mut inner: Cursor<Vec<u8>>) -> Self {
        let signature = inner.get_u32();
        let version = inner.get_u8();
        let format_version = inner.get_u8();
        let endianness = inner.get_u8();
        let int_size = inner.get_u8();
        let t_size = inner.get_u8();
        let instruction_size = inner.get_u8();
        let l_num_size = inner.get_u8();
        let integral_flag = inner.get_u8();

        assert_eq!(
            signature, 0x1B4C7561,
            "incorrect chunk signature {}",
            signature
        );
        assert_eq!(version, 0x51, "incorrect chunk version {}", version);
        assert_eq!(
            format_version, 0,
            "incorrect chunk format version {}",
            format_version
        );

        assert_eq!(
            instruction_size, 4,
            "incorrect chunk instruction size {}",
            instruction_size
        );

        println!("endianness {}", endianness);
        println!("size_t {}", t_size);
        println!("int_size {}", int_size);
        println!("l_num_size {}", l_num_size);
        println!("integral_flag {}", integral_flag);

        Self {
            inner,

            endianness,
            int_size,
            t_size,
            l_num_size,
            integral_flag,
        }
    }

    /// Get u32 with header endianness
    pub fn get_u32_he(&mut self) -> u32 {
        match self.endianness {
            0 => {
                //Big endian
                self.inner.get_u32()
            }
            1 => {
                //Little endian
                self.inner.get_u32_le()
            }
            n => panic!("Invalid chunk endianness {}", n),
        }
    }

    /// Get i32 with header endianness
    fn get_i32_he(&mut self) -> i32 {
        match self.endianness {
            0 => {
                //Big endian
                self.inner.get_i32()
            }
            1 => {
                //Little endian
                self.inner.get_i32_le()
            }
            n => panic!("invalid chunk endianness {}", n),
        }
    }

    /// Get i64 with header endianness
    fn get_i64_he(&mut self) -> i64 {
        match self.endianness {
            0 => {
                //Big endian
                self.inner.get_i64()
            }
            1 => {
                //Little endian
                self.inner.get_i64_le()
            }
            n => panic!("invalid chunk endianness {}", n),
        }
    }

    /// Get f32 with header endianness
    fn get_f32_he(&mut self) -> f32 {
        match self.endianness {
            0 => {
                //Big endian
                self.inner.get_f32()
            }
            1 => {
                //Little endian
                self.inner.get_f32_le()
            }
            n => panic!("invalid chunk endianness {}", n),
        }
    }

    /// Get f64 with header endianness
    fn get_f64_he(&mut self) -> f64 {
        match self.endianness {
            0 => {
                //Big endian
                self.inner.get_f64()
            }
            1 => {
                //Little endian
                self.inner.get_f64_le()
            }
            n => panic!("invalid chunk endianness {}", n),
        }
    }

    pub fn get_int(&mut self) -> i64 {
        match self.int_size {
            4 => self.get_i32_he() as i64,
            8 => self.get_i64_he(),
            n => panic!("invalid chunk int_size {}", n),
        }
    }

    pub fn get_size_t(&mut self) -> i64 {
        match self.t_size {
            4 => self.get_i32_he() as i64,
            8 => self.get_i64_he(),
            n => panic!("invalid chunk t_size {}", n),
        }
    }

    pub fn get_number(&mut self) -> f64 {
        match self.integral_flag {
            0 => {
                //Floating point
                match self.l_num_size {
                    4 => self.get_f32_he() as f64,
                    8 => self.get_f64_he(),
                    n => panic!("invalid chunk l_num_size {}", n),
                }
            }
            1 => {
                //Integer
                match self.l_num_size {
                    4 => self.get_i32_he() as f64, //not ideal to cast to float
                    8 => self.get_i64_he() as f64, //not ideal to cast to float
                    n => panic!("invalid chunk l_num_size {}", n),
                }
            }
            n => panic!("Invalid chunk integral flag {}", n),
        }
    }

    pub fn get_string(&mut self) -> Option<String> {
        let len = self.get_size_t() - 1;
        if len == -1 {
            return None;
        }

        let mut ret = String::with_capacity(len as usize);

        for _ in 0..len {
            ret.push(self.inner.get_u8() as char);
        }

        let _ = self.inner.get_u8(); //Consume null terminating byte

        Some(ret)
    }
}

pub trait BReadable {
    fn read(reader: &mut BReader) -> Self;
}
