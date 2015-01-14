use std::os;
use std::io::File;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};


fn search(needle: String, haystack: Vec<char>) -> Option<(usize, usize)> {
    let needle: Vec<char> = needle.chars().collect();

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
            return None;
        }
    }

    let second_pos: &Vec<usize>;
    match indexed_haystack.get(&needle[1]) {
        None => { return None; }
        Some(ref entries) => {
            second_pos = *entries;
        }
    }
    let mut possible_steps: Vec<(usize, usize)> = Vec::new();
    for first_pos in possible_starts.iter() {
        for second_pos in second_pos.iter() {
            if first_pos < second_pos {

                let start = *first_pos;
                let step = *second_pos - *first_pos;

                if haystack.iter().skip(start).enumerate().filter(|&(idx, _)| { idx % step == 0 }).map(|(_, char)| { char })
                        .zip(needle.iter())
                        .all(|(&x, &y)| { x == y }) {
                    return Some((start, step));
                }
            }
        }
    }


    None
}

fn main() {
    let file_to_search = &os::args()[1];
    println!("Reading in {}", file_to_search);
    let haystack: Vec<char> = File::open(&Path::new(file_to_search)).read_to_string().unwrap().chars().filter(|&x| { x.is_alphabetic() }).map(|x| { x.to_lowercase() }).collect();

    let needle = (&os::args()[2]).to_string();
    println!("Looking for {}", needle);
    match search(needle, haystack) {
        None => { println!("Not found"); }
        Some((start, step)) => { println!("Found starting at {} with step of {}", start, step); }
    }
}
