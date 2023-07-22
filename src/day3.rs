use std::collections::{HashMap, HashSet};

use crate::file::read_lines;

pub fn solution(filename: &String) {
    let mut doubles: Vec<char> = vec![];

    let mut tripples: HashMap<char, u32> = HashMap::new();
    let mut count_rucksacks: u32 = 0;
    let mut part2_score: u32 = 0;
    for line in read_lines(filename).unwrap() {
        if let Ok(rucksack) = line {
            let compartment_len = rucksack.len() / 2;

            let sack1: HashSet<_> = rucksack[..compartment_len].chars().collect();
            let sack2: HashSet<_> = rucksack[compartment_len..].chars().collect();
            doubles.extend(sack1.intersection(&sack2));

            for ch in rucksack.chars().collect::<HashSet<char>>() {
                tripples.insert(ch, tripples.get(&ch).unwrap_or(&0) + 1);
            }
            count_rucksacks += 1;
            if count_rucksacks % 3 == 0 {
                // Check for every 3 rucksack what the shared item is and add value to the score
                part2_score += tripples
                    .drain()
                    .filter(|kv| kv.1 == 3)
                    .map(|kv| match kv.0.is_uppercase() {
                        true => kv.0.to_owned() as u32 - 38,
                        false => kv.0.to_owned() as u32 - 96,
                    })
                    .sum::<u32>();
            }
        }
    }

    let score: u32 = doubles
        .iter()
        .map(|&char| match char.is_uppercase() {
            true => char as u32 - 38,
            false => char as u32 - 96,
        })
        .sum::<u32>();

    println!("doubles: {:?}", score,);
    println!("tripples: {:?}", part2_score,);
}
