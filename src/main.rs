use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::io::prelude::*;
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
    //println!("Contents in chunks:");
    for s in sig_chunks.iter(){
        //println!("> {}", s);
        let chunkvec:Vec<u8> = s.as_bytes().to_vec();
        sig_xor.iter_mut()
                .zip(chunkvec.iter())
                .for_each(|(x, y)| *x ^= *y); 
    }

    //Print the result
    println!("Final 32-bit:");

    for a in sig_xor.iter(){
        print!("0{:b}",a);
    }
    sig_xor
}

fn gen_str_for_add() -> String {
    let s: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(10)
    .map(char::from)
    .collect();
    println!("{}", s);
    s
}

fn main(){
    let signature:String = "4:48+16:0:1440:mss*30,7:mss,sok,ts,nop,ws:df,ecn:0".to_string()
                .chars().filter(|c| *c != ',').collect();
    let add_str = gen_str_for_add();
    let mod_signature:String = signature.clone() + &add_str;
    let orig_sgn: Vec<u8> = xor_chunks(signature);
    println!(" .");
    let added_sgn: Vec<u8> = xor_chunks(mod_signature);
    let mut entry:String = String::from_utf8(orig_sgn).unwrap() + &" ".to_string();
    entry = entry + & String::from_utf8(added_sgn).unwrap();
    
    //Write both the strings into a file
    let path = Path::new("signatures_with_added_text.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the signatures to `file`, returns `io::Result<()>`
    match file.write_all(entry.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("\n successfully wrote to {}", display),
    }
}
