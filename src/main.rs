use rand::{distributions::Alphanumeric, Rng};

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, ErrorKind};
use std::path::Path;

use plotly::common::Mode;
use plotly::{Plot, Scatter};

/// Function to get the binary encoding of a string and break it into 32-bit chunks
/// XOR the 32-bit chunks together to get a 32-bit encoding of the input string
/// Return as a vector of bytes
fn xor_chunks(name: String) -> Vec<u8> {
    
    let mut name_in_binary="".to_string();

    // Call into_bytes() which returns a Vec<u8>, and iterate accordingly
    for character in name.clone().into_bytes() {
        name_in_binary += &format!("0{:b} ", character);
    }
    // Remove whitespace
    let sig_bin:String = name_in_binary.chars().filter(|c| !c.is_whitespace()).collect();


    // Break into 32-bit chunks
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

    // XOR the chunks together
    let mut sig_xor: Vec<u8> = vec![0u8,0,0,0];
    
    for s in sig_chunks.iter(){
        let chunkvec:Vec<u8> = s.as_bytes().to_vec();
        sig_xor.iter_mut()
                .zip(chunkvec.iter())
                .for_each(|(x, y)| *x ^= *y); 
    }
    sig_xor
}

/// Generate a random alphanumeric string of the specified length
fn gen_str_for_add(n:usize) -> String {
    let s: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(n)
    .map(char::from)
    .collect();
    s
}

/// Calculate the hamming distance between the binary representations 
/// of two input strings
/// Return a Result instance for error handling when the input strings
/// are not of the same length
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

/// Convert a vector of bytes into string for easy output
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
    }
    result
}

/// Main function logs average distances with more letters added to the signature
/// Plots average distance versus number of characters added
/// Saves average distance values to a file
fn main(){
    let signature:String = "4:48+16:0:1440:mss*30,7:mss,sok,ts,nop,ws:df,ecn:0".to_string()
                .chars().filter(|c| *c != ',').collect();
    // Write both the strings into a file
    let path = Path::new("signatures_with_added_text.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    
    // Process original signature
    let orig_sgn: Vec<u8> = xor_chunks(signature.clone());
    let orig_str = sign_to_string(orig_sgn);

    // Initialise avg distance vector
    let mut avg_dist_vector:Vec<f64> = Vec::new();

    // Initialise number of characters added vector
    let mut x_values:Vec<usize> = Vec::new();
    let base:usize = 2;

    for i in 2..=14 {    
        let mut avg_distance = 0.0;
        let exp_base = base.pow(i as u32);
        x_values.push(exp_base);
        if i>10 {
            println!("Waiting for the calculations...");
        }
        for _j in 0..1000 {
            // Get random string to add to end of original signature
            let add_str = gen_str_for_add(exp_base);
            // Get modified signature
            let mod_signature:String = signature.clone() + &add_str;
            // Process modified signature
            let added_sgn: Vec<u8> = xor_chunks(mod_signature);
            let added_str:String = sign_to_string(added_sgn);

            // Calculate Hamming distance of jth compressed modified signature from compressed original signature
            let distance = match calculate_distance(orig_str.clone(), added_str){
                Ok(distance) => distance,
                Err(err) => panic!("{}", err),
            };

            // Accumulate the distance into a sum for getting mean later
            avg_distance += distance as f64;
        }

        // Getting the mean distance and saving it for plotting
        avg_distance /= 1000.0;
        //println!("\n Average distance:{}", avg_distance);
        avg_dist_vector.push(avg_distance);

        // String to write in file
        let mut entry:String = orig_str.clone() + &" ".to_string();
        entry += & format!(" {} \n", avg_distance);
        // Write the signatures to `file`, returns `io::Result<()>`
        match file.write_all(entry.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("\n Wrote average distance: {}", avg_distance),
        }
    }

    // Plot the error data
    let trace = Scatter::new(x_values, avg_dist_vector)
        .name("trace")
        .mode(Mode::LinesMarkers);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show();
    println!("{}", plot.to_inline_html(Some("avg_dist_plot")));
}
