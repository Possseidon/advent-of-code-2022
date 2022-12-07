use itertools::Itertools;

pub fn part1(input: String) -> String {
    input
        .split("\n\n")
        .map(|calories_per_elf| {
            calories_per_elf
                .split('\n')
                .filter(|calories| !calories.is_empty())
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .split("\n\n")
        .map(|calories_per_elf| {
            calories_per_elf
                .split('\n')
                .filter(|calories| !calories.is_empty())
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum::<u32>()
        .to_string()
}
