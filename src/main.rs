#[macro_use] extern crate log;

use std::num::{Int, FromPrimitive};
use std::os;
use std::io::File;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::iter::range_step;

struct InputSource {
    haystack: Vec<char>,
    indexed_haystack: HashMap<char, Vec<usize>>,
}

struct EDS<'a> {
    source: &'a InputSource,

    needle: Vec<char>,
    first_poses: Vec<usize>,
    first_poses_idx: usize,
    second_poses: Vec<usize>,
    second_poses_idx: usize,
}


impl<'a> Iterator for EDS<'a> {
    type Item = (usize, i64);

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
        let first_run = (self.first_poses_idx == 0 && self.second_poses_idx == 0);

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
    fn new(haystack: Vec<char>) -> InputSource {
        let indexed_haystack = index_chars(&haystack);

        InputSource {
            haystack: haystack,
            indexed_haystack: indexed_haystack,
        }
    }

    fn search_for(&mut self, needle: Vec<char>) -> Option<EDS> {
        EDS::new(self, needle)
    }
}

impl<'a> EDS<'a> {
    fn new(source: &InputSource, needle: Vec<char>) -> Option<EDS> {
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



fn only_alphanumeric(input: String) -> Vec<char> {
    input.chars().filter(|c| { c.is_alphabetic() }).map(|c| { c.to_lowercase() }).collect()
}

//fn only_constants(input: Vec<char>) -> Vec<char> {
    //input.filter(|&c| { c.is_alphabetic() && ! ( c }).collect()
//}

fn main() {
    let file_to_search = &os::args()[1];
    println!("Reading in {}", file_to_search);
    let haystack: Vec<char> = only_alphanumeric(File::open(&Path::new(file_to_search)).read_to_string().unwrap());
    let source = InputSource::new(haystack);
        

    let needle = (&os::args()[2]).to_string();
    println!("Looking for {}", needle);
    let needle = only_alphanumeric(needle);

    match EDS::new(&source, needle) {
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
