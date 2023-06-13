use std::cmp::min;

use super::sign::Sign;

#[derive(Debug, Clone, PartialEq)]
pub struct SignCompare{
    pub sign1: Sign,
    pub sign2: Sign,
    pub distance: usize,
    pub similarity: f64,
}

impl SignCompare{
    pub fn new(signature1: String, signature2: String) -> Self {
        let sign1 = Sign::new(signature1);
        let sign2 = Sign::new(signature2);

        let mut sc = SignCompare {
            sign1,
            sign2,
            distance : 0,
            similarity: 0.0,
        };

        if sc.sign1.len > 0 || sc.sign2.len > 0 {
            sc.distance = sc.get_distance();
            sc.similarity = sc.get_similarity();
        }
        sc
    }

    pub fn get_distance(&mut self) -> usize {
        let s1 = self.sign1.word_bin.clone();
        let s2 = self.sign2.word_bin.clone();
        
        calculate_levenshtein_distance(s1, s2)
    }
    
    pub fn get_similarity(&mut self) -> f64 {
        let s1 = self.sign1.xor_word.clone();
        let s2 = self.sign2.xor_word.clone();

        calculate_jaccard_similarity(s1, s2)
        //calculate_overlap_coefficient(s1, s2)
        //let sim = 1.0 - (calculate_levenshtein_distance(s1, s2) as f64 / 32.0);
        //sim
    }

}

/// Calculate the Levenshtein distance between the binary representations 
/// of two input strings and return the distance as integer
pub fn calculate_levenshtein_distance(s1: String, s2: String) -> usize {
    let mut dist = 0;
    if s1.len() == 0 {
        dist = s2.len();
    }
    else if s2.len() == 0 {
        dist = s1.len();
    }
    else {
        if s1.len() == s2.len() {
            for (c1, c2) in s1.chars().zip(s2.chars()) {
                if c1 != c2 {
                    dist += 1;
                }
            }
        }
        else if s1.chars().nth(0) == s2.chars().nth(0) {
            dist = calculate_levenshtein_distance(s1.chars().skip(1).collect(), s2.chars().skip(1).collect());
        }
        else {
            let tail1:String = s1.chars().skip(1).collect();
            let tail2:String = s2.chars().skip(1).collect();
        
            let d1 = calculate_levenshtein_distance(s1, tail1.clone());
            let d2 = calculate_levenshtein_distance(tail2.clone(), s2);
            let d3 = calculate_levenshtein_distance(tail1, tail2);
            dist = 1 + min(d3, min(d1, d2));
        }
    }

    dist
}


pub fn calculate_jaccard_similarity(s1: String, s2: String) -> f64 {
    let mut common_ones = 0.0;
    let mut total_ones = 0.0;
    let mut similarity = 0.0;
    if s1.len() == s2.len() {
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if (c1 == '1') && (c2 == '1') {
                common_ones += 1.0;
                total_ones += 2.0;
            }
            else if (c1 == '1') || (c2 == '1') {
                total_ones += 1.0;
            }
        }
        similarity = common_ones / total_ones;
    }

    similarity
}
 
/*
pub fn calculate_overlap_coefficient(s1: String, s2: String) -> f64 {
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