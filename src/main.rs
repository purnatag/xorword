//use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;
//use std::io::{self, ErrorKind};
use std::path::Path;

use plotly::common::Mode;
use plotly::{Plot, Scatter};

pub mod cmp_signs;
pub mod sign;

use crate::cmp_signs::*;

/// Generates a random alphanumeric string of length n where n is less than signature length
/// Randomly identifies n indices in the signature and inserts characters from the random
/// string in those positions.
fn gen_added_signature(n: usize, signature: &str) -> String {
    let mut result_vec = signature.chars().collect::<Vec<_>>();
    //println!("Size of result_vec: {}", result_vec.len());
    //let mut rng = rand::thread_rng();
    //let rand_string = Alphanumeric.sample_string(&mut rng, n);
    let range = signature.len() as usize;

    for _ in 0..n {
        // Get random index in the signature
        let idx = rand::thread_rng().gen_range(0..range);
        // Get random ASCII character to insert
        let rand_char = rand::thread_rng().gen_range(33..127) as u8 as char;
        // Insert char at position idx in the signature
        debug_assert!(idx < result_vec.len());
        result_vec.insert(idx, rand_char);
    }
    result_vec.iter().collect::<String>()
}

/// Main function logs average distances with more letters added to the signature
/// Plots average distance versus number of characters added
/// Saves average distance values to a file
fn main() {
    let signature: String = "4:48+16:0:1440:mss*30,7:mss,sok,ts,nop,ws:df,ecn:0"
        //let signature:String = "Despite being a herbivore, the Panda's digestive system remarkably resembles a carnivore's"
        .to_string()
        .chars()
        .filter(|c| *c != ',')
        .collect();
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
    let mut avg_dist_vector: Vec<f64> = Vec::new();
    let mut avg_jac_sim_vector: Vec<f64> = Vec::new();
    let mut avg_lev_sim_vector: Vec<f64> = Vec::new();
    let mut avg_gaps = 0.0;
    let total = ((num_iter - 1) * 1000) as f64;

    for i in 1..num_iter {
        println!("Step i={i}");
        let mut avg_distance = 0.0;
        let mut avg_jac_similarity = 0.0;
        let mut avg_lev_similarity = 0.0;

        //x_values.push((i * 100) as f64 / total_len as f64);
        for _ in 0..1000 {
            // Get randomly modified signature
            //println!(" j={j}");
            let added_signature = gen_added_signature(i, &signature);
            println!(
                " Added signature {added_signature} length {}",
                added_signature.len()
            );

            // let added_signature = signature.clone() + &'0'.to_string();
            // Process original and modified signature
            let sc = SignCompare::new(signature.clone(), added_signature);

            // Accumulate the distance and similarity into a sum for getting mean later
            avg_distance += sc.distance;
            avg_jac_similarity += sc.similarity_jac * 100f64;
            avg_lev_similarity += sc.similarity_lev;
            avg_gaps += sc.gaps as f64;
        }

        // Getting the mean distance and saving it for plotting
        avg_distance /= 1000f64;
        avg_jac_similarity /= 1000f64;
        avg_lev_similarity /= 1000f64;

        avg_dist_vector.push(avg_distance);
        avg_jac_sim_vector.push(avg_jac_similarity);
        avg_lev_sim_vector.push(avg_lev_similarity);

        // String to write in file
        let entry = format!(" {}, {} \n", avg_distance, avg_lev_similarity);
        // Write the signatures to `file`, returns `io::Result<()>`
        match file.write_all(entry.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!(
                "\n Wrote % change in distance: {}, % similarity: {}",
                avg_distance, avg_lev_similarity
            ),
        }
    }

    avg_gaps /= total;
    println!("Average gap between original and encoding: {}", avg_gaps);

    // Plotting the performance of the 32-byte encoding
    // using the average distance and average similarity measures
    let trace_1 = Scatter::new(avg_dist_vector.clone(), avg_jac_sim_vector.clone())
        .name("Gaps")
        .mode(Mode::LinesMarkers);
    let trace_2 = Scatter::new(avg_dist_vector.clone(), avg_lev_sim_vector.clone())
        .name("Levenshtein Similarity")
        .mode(Mode::LinesMarkers);
    let mut plot_1 = Plot::new();
    plot_1.add_traces(vec![trace_1, trace_2]);

    plot_1.show();
    println!("{}", plot_1.to_inline_html(Some("avg_dist_avg_sim_plot")));
}
