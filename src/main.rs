use rand::{distributions::Alphanumeric, Rng};

use std::fs::File;
use std::io::prelude::*;
//use std::io::{self, ErrorKind};
use std::path::Path;

use plotly::common::Mode;
use plotly::{Plot, Scatter};

pub mod sign;
pub mod cmp_signs;

use crate::cmp_signs::*;


/// Generate a random alphanumeric string of the specified length
fn gen_str_for_add(n:usize) -> String {
    let s: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(n)
    .map(char::from)
    .collect();
    s
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

    // Initialise avg distance vector
    let mut avg_dist_vector:Vec<f64> = Vec::new();
    let mut avg_sim_vector:Vec<f64> = Vec::new();

    // Initialise number of characters added vector
    let mut x_values:Vec<usize> = Vec::new();
    let base:usize = 2;

    println!("Waiting for the calculations...");
    for i in 2..=14 {    
        let mut avg_distance = 0.0;
        let mut avg_similarity = 0.0;
        let exp_base = base.pow(i as u32);
        x_values.push(exp_base);
        
        for _j in 0..1000 {
            // Get random string to add to end of original signature
            let add_str = gen_str_for_add(exp_base);
            // Get modified signature
            let mod_signature:String = signature.clone() + &add_str;
            // Process original and modified signature
            let sc = SignCompare::new(signature.clone(), mod_signature);

            let distance = (sc.distance as f64 * 100.0)/sc.sign1.len_bin as f64; 
            // Accumulate the distance and similarity into a sum for getting mean later
            avg_distance += distance as f64;
            avg_similarity += sc.similarity * 100.0;
        }

        // Getting the mean distance and saving it for plotting
        avg_distance /= 1000.0;
        avg_similarity /= 1000.0;
        
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
