#[macro_use] extern crate log;

use std::num::{Int, FromPrimitive};
use std::os;
use std::io::File;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::iter::range_step;

struct EDS {
    needle: Vec<char>,
    haystack: Vec<char>,
    indexed_haystack: HashMap<char, Vec<usize>>,
    first_pos: Vec<usize>,
    second_pos: Vec<usize>,
}


impl Iterator for EDS {
    type Item = (usize, i64);

    fn next(&mut self) -> Option<(usize, i64)> {
        None
    }
}

impl EDS {
    fn find_first_second_pos(&mut self) {
        self.indexed_haystack = HashMap::new();
        for (idx, &c) in self.haystack.iter().enumerate() {
            match self.indexed_haystack.entry(c) {
                Vacant(entry) => { entry.insert(vec![idx]); },
                Occupied(mut entry) => { (*entry.get_mut()).push(idx); }
            }
        }

        match self.indexed_haystack.get(&self.needle[0]) {
            Some(ref entries) => {
                self.first_pos = (**entries).clone();
            },
            None => {
                self.first_pos = vec![];
                return
            }
        }

        match self.indexed_haystack.get(&self.needle[1]) {
            None => { self.second_pos = vec![]; }
            Some(ref entries) => {
                self.second_pos = (**entries).clone();
            }
        }

    }
}



fn search(needle: String, haystack: Vec<char>) -> Vec<(usize, i64)> {
    let needle: Vec<char> = needle.chars().collect();
    let len_needle = needle.len() as i64;
    let len_haystack: usize = haystack.len();
    let mut results : Vec<(usize, i64)> = Vec::new();

    let mut indexed_haystack = HashMap::new();
    for (idx, c) in haystack.iter().enumerate() {
        match indexed_haystack.entry(c) {
            Vacant(entry) => { entry.insert(vec![idx]); },
            Occupied(mut entry) => { (*entry.get_mut()).push(idx); }
        }
    }

    let possible_starts: &Vec<usize>;
    match indexed_haystack.get(&needle[0]) {
        Some(ref entries) => {
            possible_starts = *entries;
        },
        None => {
            return results;
        }
    }

    let second_pos: &Vec<usize>;
    match indexed_haystack.get(&needle[1]) {
        None => { return results; }
        Some(ref entries) => {
            second_pos = *entries;
        }
    }
    for first_pos in possible_starts.iter() {
        for second_pos in second_pos.iter() {
            debug!("first_pos {} second_pos {}", first_pos, second_pos);
            let start = *first_pos;
            let step: i64 = (*second_pos as i64) - (*first_pos as i64);
            if step == 1 {
                continue;
            }
            let positions: Vec<i64> = range_step(start as i64, step*len_needle, step).collect();
            debug!("Start {} step {}", start, step);
            if positions.iter().any(|&x| { x<0 }) {
                // TODO there's probably a better maths way to do this
                break;
            }
            let positions: Vec<usize> = positions.iter().map(|&x| { FromPrimitive::from_i64(x).unwrap() }).collect();
            debug!("positions {:?}", positions);
            if positions.len() == 0 {
                // WTF
                continue;
            }

            if positions[positions.len()-1] > len_haystack - 1 {
                break;
            }
            if positions.iter().map(|&c| { &haystack[c] }).zip(needle.iter()).all(|(&a, &b)| { a == b }) {
                println!("Found starting at {} with step of {}", start, step);
                results.push((start, step));
            }
        }
    }


    results
}

fn positions<T: Int>(start: T, step: T, len: T) -> Vec<T> {
    range(Int::zero(), len).map(|i| { start + step*i } ).collect()
}

fn only_alphanumeric(input: Vec<char>) -> Vec<char> {
    input.iter().filter(|&c| { c.is_alphabetic() }).map(|&c| { c.to_lowercase() }).collect()
}

//fn only_constants(input: Vec<char>) -> Vec<char> {
    //input.filter(|&c| { c.is_alphabetic() && ! ( c }).collect()
//}

fn main() {
    let file_to_search = &os::args()[1];
    println!("Reading in {}", file_to_search);
    let haystack: Vec<char> = File::open(&Path::new(file_to_search)).read_to_string().unwrap().chars().filter(|&x| { x.is_alphabetic() }).map(|x| { x.to_lowercase() }).collect();

    let needle = (&os::args()[2]).to_string();
    println!("Looking for {}", needle);
    for &(start, step) in search(needle, haystack).iter() {
        println!("Found starting at {} with step of {}", start, step);
    }
}
