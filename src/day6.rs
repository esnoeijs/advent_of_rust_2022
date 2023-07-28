use std::collections::HashSet;

use crate::file::read_lines;

pub fn solution(filename: &String) {
    println!("part1: {:?}", find_unique_seq(filename, 4));
    println!("part2: {:?}", find_unique_seq(filename, 14));
}

fn find_unique_seq(filename: &String, num_chars: usize) -> Option<usize> {
    for line in read_lines(filename).unwrap().map(|x| x.unwrap()) {
        for n in num_chars..line.len() {
            let window: &HashSet<_> = &line[n - num_chars..n].chars().into_iter().collect();
            if window.len() == num_chars {
                return Some(n);
            }
        }
    }
    None
}
