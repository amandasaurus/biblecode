#![feature(rustc_private)]
#[macro_use] extern crate log;

extern crate ansi_term;
use ansi_term::Colour::Red;
use ansi_term::Style;

extern crate num;
use num::traits::Num;

use std::fs::File;
use std::io::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

/// Returns a Vec, of `len` numbers from `start`, stepping by `step`.
fn step_by<T: Num+Copy>(start: T, step: T, len: usize) -> Vec<T> {
    let mut result: Vec<T> = Vec::with_capacity(len);
    let mut curr = start;
    for _ in 1..len+1 {
        result.push(curr);
        curr = curr + step;
    }
    result
}
        


/// Converts a Vec<char> into a String
fn vec_to_string(input: &Vec<char>) -> String {
    input.iter().map(|&c| { c }).collect()
}

/// Index source
struct InputSource {
    /// The input 'string' (the haystack we want to look in)
    haystack: Vec<char>,

    /// The `haystack`, but indexed
    indexed_haystack: HashMap<char, Vec<usize>>,
}

/// Index the chars in `input`.
fn index_chars(input: &Vec<char>) -> HashMap<char, Vec<usize>> {
    let mut indexed_haystack = HashMap::new();
    for (idx, &c) in input.iter().enumerate() {
        match indexed_haystack.entry(c) {
            Vacant(entry) => { entry.insert(vec![idx]); },
            Occupied(mut entry) => { (*entry.get_mut()).push(idx); }
        }
    }

    indexed_haystack
}

impl InputSource {
    /// Create & Return a new InputSource based on this input haystack
    fn new(haystack: Vec<char>) -> InputSource {
        let indexed_haystack = index_chars(&haystack);

        InputSource {
            haystack: haystack,
            indexed_haystack: indexed_haystack,
        }
    }

    fn search_for_anywhere<'a>(&'a self, needle: &'a Vec<char>) -> Option<EDS> {
        EDS::new(self, needle)
    }

    fn search_for<'a>(&'a self, needle: &'a Vec<char>, start_options: Vec<usize>, step_options: Vec<i64>) -> Option<EDS> {
        EDS::new(self, needle)
    }

    /// Length of the input source
    fn source_len(&self) -> usize {
        self.haystack.len()
    }
}

// This is a struct for doing the searching
struct EDS<'a, 'b> {
    /// Input source that's used
    source: &'a InputSource,

    /// String to search for
    needle: &'b Vec<char>,

    /// Vec of indexes, within source's haystack, of the first letter of the key
    first_poses: Vec<usize>,

    /// Vec of indexes, within source's haystack, of the second letter of the key
    second_poses: Vec<usize>,

    /// Which index within `first_poses` are we at?
    first_poses_idx: usize,
    /// Which index within `second_poses` are we at?
    second_poses_idx: usize,

    
}


impl<'a, 'b> Iterator for EDS<'a, 'b> {
    // (start, offset)
    type Item = (usize, i64);

    // This actually does the searching
    fn next(&mut self) -> Option<(usize, i64)> {
        debug!("Start of next call");
        let len_first_poses = self.first_poses.len();
        let len_second_poses = self.second_poses.len();
        
        if self.first_poses_idx == len_first_poses {
            debug!("Reached end, finishing");
            // reached the end
            return None;
        }

        let len_needle = self.needle.len();
        let len_haystack = self.source.haystack.len();
        let mut have_done_one_loop = false;
        let first_run = self.first_poses_idx == 0 && self.second_poses_idx == 0;

        loop {
            debug!("Start of loop");
            // Starts off at 0/0, and we don't want to increment it the first time
            if ! (self.first_poses_idx == 0 && self.second_poses_idx == 0 && !have_done_one_loop) {
                debug!("Incrementing counters");
                if self.second_poses_idx == len_second_poses - 1 {
                    self.second_poses_idx = 0;
                    self.first_poses_idx += 1;
                } else{
                    self.second_poses_idx += 1;
                }
            }
            have_done_one_loop = true;
            if self.first_poses_idx == len_first_poses {
                debug!("Reached end, finishing");
                // reached the end
                return None;
            }

            debug!("first_poses_idx {} second_poses_idx {}", self.first_poses_idx, self.second_poses_idx);
            let first_pos = self.first_poses[self.first_poses_idx];
            let second_pos = self.second_poses[self.second_poses_idx];
            debug!("first_pos {} second_pos {}", first_pos, second_pos);

            let step: i64 = (second_pos as i64) - (first_pos as i64);
            if step == 1 {
                continue;
            }
            let positions: Vec<i64> = step_by(first_pos as i64, step, len_needle);
            //let positions: Vec<i64> = range_step(first_pos as i64, step*len_needle, step).collect();
            debug!("Start {} step {}", first_pos, step);
            if positions.iter().any(|&x| { x<0 }) {
                // TODO there's probably a better maths way to do this
                continue;
            }
            let positions: Vec<usize> = positions.iter().map(|&x| {  x as usize }).collect();
            debug!("positions {:?}", positions);
            if positions.len() == 0 {
                // WTF
                continue;
            }

            if positions[positions.len()-1] > len_haystack - 1 {
                debug!("Past end");
                continue;
            }
            if positions.iter().map(|&c| { &self.source.haystack[c] }).zip(self.needle.iter()).all(|(&a, &b)| { a == b }) {
                //println!("Found starting at {} with step of {}", first_pos, step);
                return Some((first_pos, step));
            }
        }
        None
    }
}


impl<'a, 'b> EDS<'a, 'b> {
    /// Given a `source` and `needle` to search for, return something that'll search for that. If
    /// needle is <3 char long, it returns None
    fn new(source: &'a InputSource, needle: &'b Vec<char>) -> Option<EDS<'a, 'b>> {
        if needle.len() < 3 {
            return None;
        }
        let first_poses;
        match source.indexed_haystack.get(&needle[0]) {
            None => { return None ; }
            Some(ref entries) => {
                first_poses = (**entries).clone();
            }
        }

        let second_poses;
        match source.indexed_haystack.get(&needle[1]) {
            None => { return None ; }
            Some(ref entries) => {
                second_poses = (**entries).clone();
            }
        }

        let result = EDS {
            needle: needle, source: source,
            first_poses: first_poses, second_poses: second_poses,
            first_poses_idx: 0, second_poses_idx: 0,
        };

        return Some(result);

    }
}

fn print_results(source: &InputSource, start: usize, step: i64, len: usize) {
    let total_rows = (len + 2) as i64;
    let width: usize = 31; // 15 + 1 + 15
    let start_i = start as i64;
    let source_len = source.source_len();

    for row_num in 0..total_rows {
        for idx in 0..(width as i64) {
            let source_idx = (row_num-1)*step + start_i - 15 + idx;
            let this_char = if (source_idx < 0) || source_idx >= source_len as i64 { ' ' } else { source.haystack[source_idx as usize] };
            if idx == 15 && row_num != 0 && row_num != total_rows -1 {
                print!("{}", Red.paint(this_char.to_string()));
            } else {
                print!("{}", Style::default().paint(this_char.to_string()));
            }
        }
        print!("\n");

    }
    print!("\n");

    

}


fn only_alphanumeric(input: String) -> Vec<char> {
    input.chars().filter(|c| { c.is_alphabetic() }).map(|c| { c.to_lowercase().next().unwrap() }).collect()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let file_to_search = &args[1];
    println!("Reading in {}", file_to_search);
    let mut file_contents: String = String::new();
    File::open(file_to_search).unwrap().read_to_string(&mut file_contents);
    let haystack: Vec<char> = only_alphanumeric(file_contents);
    let source = InputSource::new(haystack);
        

    let needle = (&args[2]).to_string();
    println!("Looking for {}", needle);
    let needle = only_alphanumeric(needle);
    let len_needle = needle.len();

    let has_second_arg = args.len() == 4;
    let needle2;
    let len_needle2;
    if has_second_arg {
        needle2 = only_alphanumeric((&args[3]).to_string());
        let needle2_string: String = vec_to_string(&needle2);
        println!("Looking also for {}", needle2_string);
        len_needle2 = needle2.len();
    } else {
        needle2 = vec![];
    }

    match source.search_for_anywhere(&needle) {
        None => {
            println!("Cannot look for this string");
        }
        Some(eds_searcher) => {
            for (start, step) in eds_searcher.take(10) {
                println!("Found starting at {} with step of {}", start, step);
                print_results(&source, start, step, len_needle);
                if has_second_arg {
                    // now try to find the second needle2
                    match source.search_for_anywhere(&needle2) {
                        None => {
                            println!("Cannot look for {:?}", vec_to_string(&needle2));
                        }
                        Some(needle2_searcher) => {
                            for (start2, step2) in needle2_searcher.take(1) {
                                println!("Found {:?} starting at {} with step of {}", vec_to_string(&needle2), start2, step2);

                            }
                        }
                    }
                }
            }
        }
    }


}

#[test]
fn test_step_by() {
    assert_eq!(step_by(10, 2, 3), vec![10, 12, 14]);
    assert_eq!(step_by(10, -1, 3), vec![10, 9, 8]);
}

#[test]
fn test_only_alphanumeric() {
    assert_eq!(only_alphanumeric("hello there".to_string()), "hellothere".chars().collect::<Vec<char>>());
    assert_eq!(only_alphanumeric("hello there 123".to_string()), "hellothere".chars().collect::<Vec<char>>());
    assert_eq!(only_alphanumeric("HELLO there 123".to_string()), "hellothere".chars().collect::<Vec<char>>());
}
