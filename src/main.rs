use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;
//use std::io::{self, ErrorKind};
use std::path::Path;

use plotly::common::Mode;
use plotly::{Plot, Scatter};

pub mod sign;
pub mod cmp_signs;

use crate::cmp_signs::*;


/// Generates a random alphanumeric string of length n where n is less than signature length
/// Randomly identifies n indices in the signature and inserts characters from the random
/// string in those positions.
fn gen_added_signature(n:usize, signature: String) -> String {
    let mut result_vec = signature.clone().chars().collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    let rand_string = Alphanumeric.sample_string(&mut rng, n);
    let range = signature.len() as usize;

    for i in 0..n {
        // Get random index in the signature
        let idx = rng.gen_range(0..range);
        // Get random ASCII character to insert
        let rand_char = rand_string.chars().nth(i).unwrap();
        // Insert char at position idx in the signature
        result_vec.insert(idx, rand_char);
    }
    result_vec.iter().collect::<String>()
}

/// Main function logs average distances with more letters added to the signature
/// Plots average distance versus number of characters added
/// Saves average distance values to a file
fn main(){
    let signature:String = "4:48+16:0:1440:mss*30,7:mss,sok,ts,nop,ws:df,ecn:0".to_string()
                .chars().filter(|c| *c != ',').collect();
    let num_iter = signature.len();

    // Write both the strings into a file
    let path = Path::new("signatures_with_added_text.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Initialise avg distance vector
    let mut avg_dist_vector:Vec<f64> = Vec::new();
    let mut avg_sim_vector:Vec<f64> = Vec::new();

    println!("Waiting for the calculations...");
    
    for i in 1..num_iter {    
        let mut avg_distance = 0.0;
        let mut avg_similarity = 0.0;
        
        for _j in 0..50 {
            // Get randomly modified signature
            let added_signature = gen_added_signature(i, signature.clone());
            //let added_signature = signature.clone() + &'0'.to_string();
            // Process original and modified signature
            let sc = SignCompare::new(signature.clone(), added_signature);

            // Accumulate the distance and similarity into a sum for getting mean later
            avg_distance += (sc.distance as f64 * 100.0)/sc.sign1.len_bin as f64;
            avg_similarity += sc.similarity * 100.0;
        }

        // Getting the mean distance and saving it for plotting
        avg_distance /= 50.0;
        avg_similarity /= 50.0;
        
        avg_dist_vector.push(avg_distance);
        avg_sim_vector.push(avg_similarity);

        // String to write in file
        let entry = format!(" {}, {} \n", avg_distance, avg_similarity);
        // Write the signatures to `file`, returns `io::Result<()>`
        match file.write_all(entry.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("\n Wrote % change in distance: {}, % similarity: {}", avg_distance, avg_similarity),
        }
    }

    // Plot the error data
    let trace = Scatter::new(avg_dist_vector, avg_sim_vector)
        .name("trace")
        .mode(Mode::LinesMarkers);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show();
    println!("{}", plot.to_inline_html(Some("avg_dist_plot")));
}
