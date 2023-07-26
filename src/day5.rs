use regex::Regex;
use std::collections::HashMap;

use crate::file::read_lines;

pub fn solution(filename: &String) {
    solve(filename, &part1_algo);
    print!("\n");
    solve(filename, &part2_algo);
}

fn solve(filename: &String, algo: &dyn Fn(String, &mut HashMap<usize, Vec<char>>)) {
    let mut temp: HashMap<usize, Vec<char>> = HashMap::new();
    let mut build_columns = true;

    for line in read_lines(filename).unwrap().map(|x| x.unwrap()) {
        if line.eq("") {
            build_columns = false;
            for column in temp.values_mut() {
                column.reverse();
            }
            continue;
        }

        if build_columns {
            let columns = 1 + line.len() / 4;
            for column_nr in 1..columns + 1 {
                let idx: usize = ((column_nr - 1) * 4) + 1;

                let item = line.chars().nth(idx).unwrap();
                if item.is_alphabetic() {
                    temp.entry(column_nr).or_insert(vec![]).push(item);
                }
            }
        } else {
            algo(line, &mut temp);
        }
    }

    for nr in 1..temp.len() + 1 {
        print!("{:?}", temp.get(&nr).unwrap().last().unwrap());
    }
}

fn part1_algo(line: String, temp: &mut HashMap<usize, Vec<char>>) {
    let re = Regex::new(r"move (?<nr>[[0-9]]+) from (?<from>[[0-9]]+) to (?<to>[[0-9]]+)").unwrap();
    let caps = match re.captures(&line) {
        Some(caps) => caps,
        None => panic!("Regex doesn't match: {:?}", line),
    };

    let nr: usize = caps["nr"].parse().expect("Failed to parse 'nr'");
    let from: usize = caps["from"].parse().expect("Failed to parse 'from'");
    let to: usize = caps["to"].parse().expect("Failed to parse 'to'");

    for _ in 0..nr {
        let value = temp.get_mut(&from).unwrap().pop().unwrap();
        temp.get_mut(&to).unwrap().push(value);
    }
}

fn part2_algo(line: String, columns: &mut HashMap<usize, Vec<char>>) {
    let re = Regex::new(r"move (?<nr>[[0-9]]+) from (?<from>[[0-9]]+) to (?<to>[[0-9]]+)").unwrap();
    let caps = match re.captures(&line) {
        Some(caps) => caps,
        None => panic!("Regex doesn't match: {:?}", line),
    };

    let nr: usize = caps["nr"].parse().expect("Failed to parse 'nr'");
    let from: usize = caps["from"].parse().expect("Failed to parse 'from'");
    let to: usize = caps["to"].parse().expect("Failed to parse 'to'");

    let column_from = columns.get_mut(&from).unwrap();
    let mut value_stack: Vec<char> = vec![];
    for _ in 0..nr {
        value_stack.push(column_from.pop().unwrap());
    }

    columns.get_mut(&to).unwrap().extend(value_stack.iter().rev());
}
