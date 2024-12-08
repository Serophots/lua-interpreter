use std::io::Cursor;

use bytes::Buf;

///Bytecode reader which reads Integer, String bytecode primatives from headers
pub struct BReader {
    pub inner: Cursor<Vec<u8>>,

    pub endianness: u8,    // 0 for high, 1 for low
    pub integral_flag: u8, // 0 for floating point, 1 for integers

    pub c_int_size: u8,
    pub c_size_t: u8,
    pub lua_int_size: u8,
    pub lua_num_size: u8,
}
impl BReader {
    /// https://www.lua.org/source/5.3/ldump.c.html
    pub fn from_headers(mut inner: Cursor<Vec<u8>>) -> Self {
        //Read signature, version & format version
        let signature = inner.get_u32(); //4 bytes
        let version = inner.get_u8(); //byte
        let format_version = inner.get_u8(); //byte

        assert_eq!(
            signature, 0x1B4C7561,
            "incorrect chunk signature {:#x}",
            signature
        );
        assert_eq!(version, 0x53, "incorrect chunk version {:#x}", version);
        assert_eq!(
            format_version, 0,
            "incorrect chunk format version {:#x}",
            format_version
        );

        // Read LUAC_DATA - 6 bytes
        let luac_data = [
            inner.get_u8(), //Not sure what this is, a comment in the src says its to stop conversion errors?
            inner.get_u8(),
            inner.get_u8(),
            inner.get_u8(),
            inner.get_u8(),
            inner.get_u8(),
        ];

        assert_eq!(
            luac_data,
            [0x19, 0x93, 0x0d, 0x0a, 0x1a, 0x0a],
            "incorrect luac_data {:#?}",
            luac_data
        );

        let c_int_size = inner.get_u8(); //byte   in C: sizeof(int)
        let c_size_t = inner.get_u8(); //byte   in C: sizeof(size_t)
        let instruction_size = inner.get_u8(); //byte   in C: sizeof(Instruction)
        let lua_int_size = inner.get_u8(); //byte   in C: sizeof(lua_Integer), int64 on windows
        let lua_num_size = inner.get_u8(); //byte   in C: sizeof(lua_Number),  seems to be a double

        println!("c_int_size {}", c_int_size);
        println!("c_size_t {}", c_size_t);
        println!("instruction_size {}", instruction_size);
        println!("lua_int_size {}", lua_int_size);
        println!("lua_num_size {}", lua_num_size);

        assert_eq!(instruction_size, 4);

        let mut reader = Self {
            inner,

            endianness: 1, //Not sure how this is properly determine these
            integral_flag: 0,

            c_int_size,
            c_size_t,
            lua_int_size,
            lua_num_size,
        };

        //LUAC bits?
        let luac_int = reader.get_c_int(); //DumpInteger takes a c 'int' type
        let luac_num = reader.get_lua_number(); //DumpNumber takes lua_Number

        println!("luac_int {:#x}", luac_int);
        println!("luac_num {}", luac_num);

        assert_eq!(luac_int, 0x5678, "LUAC_INT constant incorrect");
        // assert_eq!(luac_num, 370.5, "LUAC_NUM constant incorrect");

        let _size_upvalues = reader.get_byte(); //not sure why this is between the header and the first proto

        reader
    }

    /// Get u32 with header endianness
    pub fn get_u32(&mut self) -> u32 {
        match self.endianness {
            0 => {
                //Big endian
                self.inner.get_u32()
            }
            1 => {
                //Little endian
                self.inner.get_u32_le()
            }
            n => panic!("Invalid endianness {}", n),
        }
    }

    /// Get i32 with header endianness
    fn get_i32(&mut self) -> i32 {
        match self.endianness {
            0 => {
                //Big endian
                self.inner.get_i32()
            }
            1 => {
                //Little endian
                self.inner.get_i32_le()
            }
            n => panic!("invalid endianness {}", n),
        }
    }

    /// Get i64 with header endianness
    fn get_i64(&mut self) -> i64 {
        match self.endianness {
            0 => {
                // Big endian
                self.inner.get_i64()
            }
            1 => {
                //Little endian
                self.inner.get_i64_le()
            }
            n => panic!("invalid endianness {}", n),
        }
    }

    /// Get f32 with header endianness
    fn get_f32(&mut self) -> f32 {
        match self.endianness {
            0 => {
                //Big endian
                self.inner.get_f32()
            }
            1 => {
                // Little endian
                self.inner.get_f32_le()
            }
            n => panic!("invalid endianness {}", n),
        }
    }

    /// Get f64 with header endianness
    fn get_f64(&mut self) -> f64 {
        match self.endianness {
            0 => {
                //Big endian
                self.inner.get_f64()
            }
            1 => {
                // Little endian
                self.inner.get_f64_le()
            }
            n => panic!("invalid endianness {}", n),
        }
    }

    pub fn get_byte(&mut self) -> u8 {
        self.inner.get_u8()
    }

    pub fn remaining(&self) -> usize {
        self.inner.remaining()
    }

    pub fn get_c_int(&mut self) -> i64 {
        match self.c_int_size {
            4 => self.get_i32() as i64,
            8 => self.get_i64(),
            n => panic!("invalid chunk c_int_size {}", n),
        }
    }

    pub fn get_c_size_t(&mut self) -> i64 {
        match self.c_size_t {
            4 => self.get_i32() as i64,
            8 => self.get_i64(),
            n => panic!("invalid chunk c_size_t {}", n),
        }
    }

    pub fn get_lua_number(&mut self) -> f64 {
        match self.integral_flag {
            0 => {
                //Floating point
                match self.lua_num_size {
                    4 => self.get_f32() as f64,
                    8 => self.get_f64(),
                    n => panic!("invalid chunk l_num_size {}", n),
                }
            }
            1 => {
                //Integer
                match self.lua_num_size {
                    4 => self.get_i32() as f64, //not ideal to cast to float
                    8 => self.get_i64() as f64, //not ideal to cast to float
                    n => panic!("invalid chunk l_num_size {}", n),
                }
            }
            n => panic!("Invalid chunk integral flag {}", n),
        }
    }

    pub fn get_string(&mut self) -> Option<String> {
        let len = self.get_byte() as isize - 1;
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
