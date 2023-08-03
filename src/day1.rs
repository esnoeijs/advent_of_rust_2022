use crate::file;

pub fn solution(filename: &String) {
    let mut elf_calories = count_calories(filename);
    elf_calories.sort();
    elf_calories.reverse();

    println!("part1: {:?}", elf_calories.iter().max());
    println!("part2: {:?}", elf_calories.iter().take(3).sum::<i32>());
}

fn count_calories(filename: &String) -> Vec<i32> {
    let mut elf_calories: Vec<i32> = vec![];
    let mut elf_index = 0;
    elf_calories.push(0);
    for line in file::read_lines(filename).unwrap() {
        if let Ok(value) = line {
            match value.as_str() {
                "" => {
                    elf_index += 1;
                    elf_calories.push(0);
                }
                _ => elf_calories[elf_index] += value.parse::<i32>().unwrap(),
            }
        }
    }

    return elf_calories;
}
