#[derive(Clone, Debug, PartialEq)]
pub struct Sign {
    /// string, specifically a signature obtained from a sensor
    pub word: String,
    /// the signature in binary 
    pub word_bin: String,
    /// length of the signature
    pub len: usize,
    /// length of the binary signature
    pub len_bin: usize,
    /// 32-bit xor-ed encoding of the binary signature
    pub xor_bytes: Vec<u8>,
    /// Length 32 string representation of the xor encoding
    pub xor_word: String,
}

impl Sign{
        /// Initialises a Sign instance
    pub fn new(signature : String) -> Self {
        let word = signature.clone();
        let len = word.len();
        let mut s = Sign {
            word,
            word_bin: String::new(),
            len,
            len_bin: 0,
            xor_bytes: Vec::new(),
            xor_word: String::new(),
        };

        if s.len > 0 {
            s.word_bin = s.get_binary_string();
            s.len_bin = s.word_bin.len();
            s.xor_bytes = s.xor_chunks();
            s.xor_word = s.sign_to_string();
        }

        s
    }

    /// Convert input string to binary
    pub fn get_binary_string(&mut self) -> String {
        let mut word_in_binary= String :: new();
    
        // Call into_bytes() which returns a Vec<u8>, and iterate accordingly
        for character in self.word.clone().into_bytes() {
            word_in_binary += &format!("0{:b} ", character);
        }
        // Remove whitespace
        let word_bin_nowhitespace:String = word_in_binary.chars().filter(|c| !c.is_whitespace()).collect();
        word_bin_nowhitespace
    }

    /// Function to get the binary encoding of a string and break it into 32-bit chunks
    /// XOR the 32-bit chunks together to get a 32-bit encoding of the input string
    /// Return as a vector of bytes
    pub fn xor_chunks(&mut self) -> Vec<u8> {
        let word = self.word_bin.clone();
        // Break word_bin into 32-bit chunks
        let num_iters = self.len_bin/32;
    
        // Vec<u8> output of XORing the chunks together will be stored here
        let mut sig_xor: Vec<u8> = vec![0u8,0,0,0];
        
        for i in 0..=num_iters{
            let mut chunk:String = word.chars().skip(32*i).take(32).collect();
            if chunk.len() < 32 {
                let mut zeros = String :: new();
                for _i in 0..(32 - chunk.len()){
                    zeros += &"0".to_string();
                }
                chunk = zeros + &chunk;
            } 
            
            let chunk_to_vec:Vec<u8> = chunk.as_bytes().to_vec();
            sig_xor.iter_mut()
                    .zip(chunk_to_vec.iter())
                    .for_each(|(x, y)| *x ^= *y); 
        }
    
        //println!("xor_bytes: {:?}", &*sig_xor);
        sig_xor
    }

    /// Convert a vector of bytes into string for easy output
    pub fn sign_to_string(&mut self) -> String {
        let mut result:String = String::new();
        for s in self.xor_bytes.iter(){
            let mut str = format!("{:b}",s);
            if str.len() < 8 {
                let mut i = 8 - str.len();
                let mut zeros = String::new();
                while i > 0 {
                    zeros += &"0".to_string();
                    i -= 1;
                }
                str = zeros + &str;
            }
            result += &str;
        }
        
        result
    }
}


