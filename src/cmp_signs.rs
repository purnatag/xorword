use std::cmp::min;
use std::time::Instant;

use super::sign::Sign;

#[derive(Debug, Clone, PartialEq)]
pub struct SignCompare {
    pub sign1: Sign,
    pub sign2: Sign,
    pub distance: usize,
    pub similarity: f64,
}

impl SignCompare {
    pub fn new(signature1: String, signature2: String) -> Self {
        let sign1 = Sign::new(signature1);
        let sign2 = Sign::new(signature2);
        let mut sc = Self {
            sign1,
            sign2,
            distance: 0,
            similarity: 0.0,
        };
        println!(" Computing distance and similarity");
        if sc.sign1.len > 0 || sc.sign2.len > 0 {
            sc.distance = sc.get_distance();
            sc.similarity = sc.get_similarity();
        }
        sc
    }

    pub fn get_distance(&mut self) -> usize {
        println!(" computing distance... ");
        let inst = Instant::now();
        // let x = calculate_levenshtein_distance(&self.sign1.word_bin, &self.sign2.word_bin);
        let x = distance::levenshtein(&self.sign1.word_bin, &self.sign2.word_bin);
        println!(
            "                    ... took {} secs",
            inst.elapsed().as_secs()
        );
        x
    }

    pub fn get_similarity(&mut self) -> f64 {
        println!(" computing similarity...");
        let inst = Instant::now();
        let x = calculate_jaccard_similarity(&self.sign1.xor_word, &self.sign2.xor_word);
        println!(
            "                      .... took {} secs",
            inst.elapsed().as_secs()
        );
        x
        //calculate_overlap_coefficient(s1, s2)
        //let sim = 1.0 - (calculate_levenshtein_distance(s1, s2) as f64 / 32.0);
        //sim
    }
}

/// Head and tail of a str
fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

/// Calculate the Levenshtein distance between the binary representations
/// of two input strings and return the distance as integer
pub fn calculate_levenshtein_distance(s1: &str, s2: &str) -> usize {
    if s1.len() == 0 {
        return s2.len();
    }

    if s2.len() == 0 {
        return s1.len();
    }

    if s1.len() == s2.len() {
        let mut dist = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                dist += 1;
            }
        }
        return dist;
    }

    let (h1, t1) = car_cdr(s1);
    let (h2, t2) = car_cdr(s2);
    if h1 == h2 {
        return calculate_levenshtein_distance(t1, t2);
    }

    let d1 = calculate_levenshtein_distance(t1, s2);
    let d2 = calculate_levenshtein_distance(s1, t2);
    let d3 = calculate_levenshtein_distance(t1, t2);
    1 + min(d3, min(d1, d2))
}

pub fn calculate_jaccard_similarity(s1: &str, s2: &str) -> f64 {
    let mut common_ones = 0.0;
    let mut total_ones = 0.0;
    let mut similarity = 0.0;
    if s1.len() == s2.len() {
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if (c1 == '1') && (c2 == '1') {
                common_ones += 1.0;
                total_ones += 2.0;
            } else if (c1 == '1') || (c2 == '1') {
                total_ones += 1.0;
            }
        }
        similarity = common_ones / total_ones;
    }

    similarity
}

/*
pub fn calculate_overlap_coefficient(s1: &str, s2: &str) -> f64 {
    let mut overlap = 0.0;
    let mut similarity = 0.0;
    if s1.len() == s2.len() {
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 == c2 {
                overlap += 1.0;
            }
        }
        similarity = overlap / 32.0;
    }

    similarity
}*/
