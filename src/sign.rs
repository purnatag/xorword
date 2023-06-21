const ENCODING_SIZE: usize = 16;
#[derive(Clone, Debug, PartialEq)]
pub struct Sign {
    /// string, specifically a signature obtained from a sensor
    pub word: String,
    /// the encoded signature in binary
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
            encoding: String::new(),
            len,
        };

        if s.len > 0 {
            s.get_encoding();
        }

        s
    }

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
