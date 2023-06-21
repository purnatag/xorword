//use std::cmp::min;
//use std::time::Instant;
use super::sign::Sign;
use str_distance::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SignCompare {
    /// Wrapped version of the original p0f signature
    pub sign1: Sign,
    /// Wrapped version of the signature with random characters
    /// inserted at random positions in the original signature
    pub sign2: Sign,
    /// Levenshtein distance between the 'word' fields i.e.,
    /// unencoded ascii signatures
    pub distance: f64,
    /// Jaccard similarity between the encoded strings
    pub similarity_jac: f64,
    /// Levenshtein similarity between the encoded strings
    pub similarity_lev: f64,
}

impl SignCompare {
    pub fn new(signature1: String, signature2: String) -> Self {
        let sign1 = Sign::new(signature1);
        let sign2 = Sign::new(signature2);
        let mut sc = Self {
            sign1,
            sign2,
            distance: 0.0,
            similarity_jac: 0.0,
            similarity_lev: 0.0,
        };
        //println!(" Computing distance and similarity");
        if sc.sign1.len > 0 || sc.sign2.len > 0 {
            sc.distance = sc.get_distance();
            sc.similarity_jac = sc.get_similarity_jac();
            sc.similarity_lev = sc.get_similarity_lev();
        }
        sc
    }

    pub fn get_distance(&mut self) -> f64 {
        //println!(" computing distance... ");
        //let inst = Instant::now();
        // let x = calculate_levenshtein_distance(&self.sign1.word_bin, &self.sign2.word_bin);
        let x = str_distance::str_distance_normalized(
            &self.sign1.word,
            &self.sign2.word,
            Levenshtein::default(),
        ) * 100f64;
        //println!("                    ... took {} secs",inst.elapsed().as_secs());
        x
    }

    pub fn get_similarity_jac(&mut self) -> f64 {
        //println!(" computing similarity...");
        //let inst = Instant::now();
        let x = calculate_jaccard_similarity(&self.sign1.encoding, &self.sign2.encoding);
        //println!("                      .... took {} secs", inst.elapsed().as_secs());
        x
    }

    pub fn get_similarity_lev(&mut self) -> f64 {
        //println!(" computing distance... ");
        //let inst = Instant::now();
        // let x = calculate_levenshtein_distance(&self.sign1.word_bin, &self.sign2.word_bin);
        let x = str_distance::str_distance_normalized(
            &self.sign1.encoding,
            &self.sign2.encoding,
            Levenshtein::default(),
        ) * 100f64;
        //println!("                    ... took {} secs",inst.elapsed().as_secs());
        100f64 - x
    }
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
