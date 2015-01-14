use std::num::Int;
use std::os;
use std::io::File;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::iter::range_step;

fn search(needle: String, haystack: Vec<char>) -> Vec<(usize, i64)> {
    let needle: Vec<char> = needle.chars().collect();
    let len_needle = needle.len() as i64;
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
    let mut possible_steps: Vec<(usize, usize)> = Vec::new();
    for first_pos in possible_starts.iter() {
        for second_pos in second_pos.iter() {
            let start = *first_pos;
            let step: i64 = (*second_pos as i64) - (*first_pos as i64);
            let positions: Vec<i64> = range_step(start as i64, step*len_needle, step).collect();
            println!("Start {} step {}", start, step);
            if positions.iter().any(|&x| { x<0 }) {
                // TODO there's probably a better maths way to do this
                continue;
            }

            println!("Range {:?}", positions);
            if positions.iter().enumerate().all(|(needle_idx, haystack_idx)| { needle[needle_idx] == haystack[haystack_idx] } ) {
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
