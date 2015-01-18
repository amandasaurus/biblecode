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
    first_poses: Vec<usize>,
    first_poses_idx: usize,
    second_poses: Vec<usize>,
    second_poses_idx: usize,
}


impl Iterator for EDS {
    type Item = (usize, i64);

    fn next(&mut self) -> Option<(usize, i64)> {
        let len_first_poses = self.first_poses.len();
        let len_second_poses = self.second_poses.len();
        let len_needle = self.needle.len();
        let len_haystack = self.haystack.len();
        let mut have_done_one_loop = false;

        loop {
            // Starts off at 0/0, and we don't want to increment it the first time
            if ! (self.first_poses_idx == 0 && self.second_poses_idx == 0 && !have_done_one_loop) {
                if self.second_poses_idx == len_second_poses {
                    self.second_poses_idx = 0;
                    self.first_poses_idx += 1;
                } else{
                    self.second_poses_idx += 1;
                }
            }
            have_done_one_loop = true;

            let first_pos = self.first_poses[self.first_poses_idx];
            let second_pos = self.second_poses[self.second_poses_idx];
            debug!("first_pos {} second_pos {}", first_pos, second_pos);

            let step: i64 = (second_pos as i64) - (first_pos as i64);
            if step == 1 {
                continue;
            }
            let positions: Vec<i64> = std::iter::count(first_pos as i64, step as i64).take(len_needle).collect();
            //let positions: Vec<i64> = range_step(first_pos as i64, step*len_needle, step).collect();
            debug!("Start {} step {}", first_pos, step);
            if positions.iter().any(|&x| { x<0 }) {
                // TODO there's probably a better maths way to do this
                continue;
            }
            let positions: Vec<usize> = positions.iter().map(|&x| { FromPrimitive::from_i64(x).unwrap() }).collect();
            debug!("positions {:?}", positions);
            if positions.len() == 0 {
                // WTF
                continue;
            }

            if positions[positions.len()-1] > len_haystack - 1 {
                debug!("Past end");
                break;
            }
            if positions.iter().map(|&c| { &self.haystack[c] }).zip(self.needle.iter()).all(|(&a, &b)| { a == b }) {
                //println!("Found starting at {} with step of {}", first_pos, step);
                return Some((first_pos, step));
            }
        }
        None
    }
}

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

impl EDS {
    fn new(needle: Vec<char>, haystack: Vec<char>) -> Option<EDS> {
        let indexed_haystack = index_chars(&haystack);

        let first_poses;
        match indexed_haystack.get(&needle[0]) {
            None => { return None ; }
            Some(ref entries) => {
                first_poses = (**entries).clone();
            }
        }

        let second_poses;
        match indexed_haystack.get(&needle[1]) {
            None => { return None ; }
            Some(ref entries) => {
                second_poses = (**entries).clone();
            }
        }
                
        let result = EDS {
            needle: needle, haystack: haystack,
            indexed_haystack: indexed_haystack,
            first_poses: first_poses, second_poses: second_poses,
            first_poses_idx: 0, second_poses_idx: 0,
        };

        Some(result)
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

    let second_poses: &Vec<usize>;
    match indexed_haystack.get(&needle[1]) {
        None => { return results; }
        Some(ref entries) => {
            second_poses = *entries;
        }
    }
    for first_pos in possible_starts.iter() {
        for second_pos in second_poses.iter() {
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
                continue;
            }
            let positions: Vec<usize> = positions.iter().map(|&x| { FromPrimitive::from_i64(x).unwrap() }).collect();
            debug!("positions {:?}", positions);
            if positions.len() == 0 {
                // WTF
                continue;
            }

            if positions[positions.len()-1] > len_haystack - 1 {
                debug!("Past end");
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
    let needle: Vec<char> = needle.chars().collect();

    let eds_searcher: EDS;
    match EDS::new(needle, haystack) {
        None => {
            println!("Cannot look for this string");
        }
        Some(ref mut eds_searcher) => {
            for (start, step) in *eds_searcher {
                println!("Found starting at {} with step of {}", start, step);
            }
        }
    }


}
