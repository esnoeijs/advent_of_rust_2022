use crate::file::read_lines;

#[derive(Debug, Copy, Clone)]
struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }
}

pub fn solution(filename: &String) {
    let groups_of_pairs: Vec<[Range; 2]> = read_lines(filename)
        .unwrap_or_else(|_| panic!("cannot read file"))
        .filter_map(|line| line.ok())
        .map(|line| construct_group(&line))
        .collect();

    let mut fully_overlapping_pairs: u32 = 0;
    let mut overlapping_pairs: u32 = 0;
    for group in groups_of_pairs {
        let [group1, group2] = group;

        fully_overlapping_pairs += match [[group1, group2], [group2, group1]].iter().any(|g| {
            g.get(0).unwrap().min <= g.get(1).unwrap().min
                && g.get(0).unwrap().max >= g.get(1).unwrap().max
        }) {
            true => 1,
            false => 0,
        };

        overlapping_pairs += match [[group1, group2], [group2, group1]].iter().any(|g| {
            g.get(0).unwrap().min <= g.get(1).unwrap().max
                && g.get(0).unwrap().max >= g.get(1).unwrap().min
        }) {
            true => 1,
            false => 0,
        };
    }

    println!("fully overlapping pairs: {:?}", fully_overlapping_pairs);
    println!("overlapping pairs: {:?}", overlapping_pairs);
}

fn construct_group(line: &String) -> [Range; 2] {
    line.split(",")
        .take(2)
        .map(|pair| {
            let pair = pair
                .split("-")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            Range::new(*pair.get(0).unwrap(), *pair.get(1).unwrap())
        })
        .collect::<Vec<Range>>()
        .try_into()
        .unwrap()
}
