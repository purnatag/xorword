use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, ErrorKind};
use std::path::Path;

fn xor_chunks(name: String) -> Vec<u8> {
    
    let mut name_in_binary="".to_string();

    // Call into_bytes() which returns a Vec<u8>, and iterate accordingly
    for character in name.clone().into_bytes() {
        name_in_binary += &format!("0{:b} ", character);
    }
    //Remove whitespace
    let sig_bin:String = name_in_binary.chars().filter(|c| !c.is_whitespace()).collect();


    //Break into 32-bit chunks
    let mut sig_chunks: Vec<String>=vec![];
    let num_iters = sig_bin.len()/32 + 1;

    for i in 0..num_iters{
        let mut push_str:String = sig_bin.chars().skip(32*i).take(32).collect();
        if push_str.len() < 32 {
            let mut zeros = "".to_string();
            for _i in 0..(32 - push_str.len()){
                zeros += &"0".to_string();
            }
            push_str = zeros + &push_str;
        } 
        sig_chunks.push(push_str);
    }

    //XOR the chunks together
    let mut sig_xor: Vec<u8> = vec![0u8,0,0,0];
    
    for s in sig_chunks.iter(){
        let chunkvec:Vec<u8> = s.as_bytes().to_vec();
        sig_xor.iter_mut()
                .zip(chunkvec.iter())
                .for_each(|(x, y)| *x ^= *y); 
    }
    println!("{}",sig_xor.len());
    sig_xor
}

fn gen_str_for_add(n:usize) -> String {
    let s: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(10*n)
    .map(char::from)
    .collect();
    println!("{}", s);
    s
}

fn calculate_distance(s1:String, s2:String) -> Result<i32, io::Error> {
    if s1.len() != s2.len() /*|| s1.len() != 32*/ {
        return Err(io::Error::new(ErrorKind::InvalidInput, "Invalid arguments to calculate distance"));
    }

    let mut dist = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            dist += 1;
        }
    }

    Ok(dist)
}

fn sign_to_string(sgn:Vec<u8>) -> String {
    let mut result:String = String::new();
    for s in sgn.iter(){
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
        println!("> {}", str);
    }
    result
}

fn main(){
    let signature:String = "4:48+16:0:1440:mss*30,7:mss,sok,ts,nop,ws:df,ecn:0".to_string()
                .chars().filter(|c| *c != ',').collect();
    //Write both the strings into a file
    let path = Path::new("signatures_with_added_text.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    let orig_sgn: Vec<u8> = xor_chunks(signature.clone());
    let orig_str = sign_to_string(orig_sgn);

    for i in 1..11 {    
        let add_str = gen_str_for_add(i);
        let mod_signature:String = signature.clone() + &add_str;
        let added_sgn: Vec<u8> = xor_chunks(mod_signature);

        let mut entry:String = orig_str.clone() + &" ".to_string();
        println!("\n");
        entry = entry + & sign_to_string(added_sgn);

        let distance = match calculate_distance(entry.chars().take(32).collect(), entry.chars().skip(33).take(32).collect()){
            Ok(distance) => distance,
            Err(err) => panic!("{}", err),
        };
        println!("Distance: {}", distance);
        entry += & format!(" {} \n", distance);
        // Write the signatures to `file`, returns `io::Result<()>`
        match file.write_all(entry.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("\n successfully wrote to {}", display),
        }
    }
}
