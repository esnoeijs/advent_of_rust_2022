use crate::file::read_lines;

#[derive(Debug, Copy, Clone)]
enum RPSCodes {
    Rock,
    Paper,
    Scissors,
}

pub fn solution(filename: &String) {
    println!("part1: {:?}", part1_score(filename));
    println!("part2: {:?}", part2_score(filename));
}

fn part1_score(filename: &String) -> u32 {
    let mut score: u32 = 0;
    for line in read_lines(filename).unwrap() {
        if let Ok(value) = line {
            let [theirs, mine] = parse_codes(value, &match_code_to_move);
            score += score_game(&mine, &theirs);
        }
    }
    score
}

fn part2_score(filename: &String) -> u32 {
    let mut score: u32 = 0;
    for line in read_lines(filename).unwrap() {
        if let Ok(value) = line {
            let [theirs, mine] = parse_codes(value, &match_code_to_strategy);
            score += score_game(&mine, &theirs);
        }
    }
    score
}

fn score_game(mine: &RPSCodes, theirs: &RPSCodes) -> u32 {
    let mut score: u32 = 0;
    score += match mine {
        RPSCodes::Rock => 1,
        RPSCodes::Paper => 2,
        RPSCodes::Scissors => 3,
    };

    score += match [mine, theirs] {
        [RPSCodes::Rock, RPSCodes::Paper] => 0,
        [RPSCodes::Paper, RPSCodes::Scissors] => 0,
        [RPSCodes::Scissors, RPSCodes::Rock] => 0,
        [RPSCodes::Rock, RPSCodes::Scissors] => 6,
        [RPSCodes::Paper, RPSCodes::Rock] => 6,
        [RPSCodes::Scissors, RPSCodes::Paper] => 6,
        [RPSCodes::Rock, RPSCodes::Rock] => 3,
        [RPSCodes::Paper, RPSCodes::Paper] => 3,
        [RPSCodes::Scissors, RPSCodes::Scissors] => 3,
    };
    score
}

fn parse_codes(
    value: String,
    calculate_move: &dyn Fn(&char, Option<&RPSCodes>) -> Option<RPSCodes>,
) -> [RPSCodes; 2] {
    if value.len() < 3 {
        panic!("code is too short");
    }

    let theirs = match_code_to_move(&value.chars().nth(0).unwrap(), None).unwrap();
    let mine = value.chars().nth(2).unwrap();
    [theirs, calculate_move(&mine, Some(&theirs)).unwrap()]
}

fn match_code_to_move(code: &char, _theirs: Option<&RPSCodes>) -> Option<RPSCodes> {
    match code {
        'A' => Some(RPSCodes::Rock),
        'X' => Some(RPSCodes::Rock),
        'B' => Some(RPSCodes::Paper),
        'Y' => Some(RPSCodes::Paper),
        'C' => Some(RPSCodes::Scissors),
        'Z' => Some(RPSCodes::Scissors),
        _ => None,
    }
}

fn match_code_to_strategy(code: &char, theirs: Option<&RPSCodes>) -> Option<RPSCodes> {
    match (code, theirs.unwrap()) {
        ('X', RPSCodes::Rock) => Some(RPSCodes::Scissors),
        ('X', RPSCodes::Paper) => Some(RPSCodes::Rock),
        ('X', RPSCodes::Scissors) => Some(RPSCodes::Paper),
        ('Y', RPSCodes::Rock) => Some(RPSCodes::Rock),
        ('Y', RPSCodes::Paper) => Some(RPSCodes::Paper),
        ('Y', RPSCodes::Scissors) => Some(RPSCodes::Scissors),
        ('Z', RPSCodes::Rock) => Some(RPSCodes::Paper),
        ('Z', RPSCodes::Paper) => Some(RPSCodes::Scissors),
        ('Z', RPSCodes::Scissors) => Some(RPSCodes::Rock),
        _ => None,
    }
}
