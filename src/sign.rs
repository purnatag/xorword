//use flate2::write::ZlibEncoder;
//use flate2::Compression;
//use std::io::prelude::Write;

const ENCODING_SIZE: usize = 16;

#[derive(Clone, Debug, PartialEq)]
pub struct Sign {
    /// string, specifically a signature obtained from a sensor
    pub word: String,
    /// compress the word using flate2
    //pub flate_word: String,
    /// the encoded compressed signature in binary
    pub encoding: String,
    /// length of the signature
    pub len: usize,
}

impl Sign {
    /// Initialises a Sign instance
    pub fn new(signature: String) -> Self {
        let word = signature.clone();
        let len = word.len();
        let mut s = Sign {
            word,
            //flate_word: String::new(),
            encoding: String::new(),
            len,
        };

        if s.len > 0 {
            //s.compress();
            s.get_encoding();
        }

        s
    }

    /*
    /// compress signature with flate2 crate
    pub fn compress(&mut self) {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(self.word.as_bytes());
        let compressed_bytes = encoder.finish();

        let mut cb: Vec<u8> = Vec::new();
        match compressed_bytes {
            Ok(compressed_bytes) => cb = compressed_bytes,
            Err(e) => println!("Error: {e:?}"),
        };

        let stringflate = String::from_utf8(cb.clone());
        match stringflate {
            Ok(s) => self.flate_word = s,
            Err(e) => println!("Error: {e:?}"),
        }
    }*/

    /// Get the 32 byte encoding of the signature
    pub fn get_encoding(&mut self) {
        let mut xor_counter = 0;
        let mut xor = [0u8; ENCODING_SIZE];
        for ch in self.word.chars() {
            xor[xor_counter % ENCODING_SIZE] ^= ch as u8;
            xor_counter += 1;
        }

        let mut result = String::new();
        for i in 0..ENCODING_SIZE {
            let mut add = format!("0{:b}", xor[i]);
            let mut add_len = add.len();
            while add_len < ENCODING_SIZE {
                add += &"0".to_string();
                add_len += 1;
            }
            result.push_str(&add);
        }
        assert!(result.len() == 256);
        self.encoding = result;
    }
}
