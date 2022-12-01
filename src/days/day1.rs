use itertools::Itertools;

pub fn part1(input: String) -> u32 {
    input
        .split("\n\n")
        .map(|calories_per_elf| {
            calories_per_elf
                .split('\n')
                .filter(|calories| !calories.is_empty())
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

pub fn part2(input: String) -> u32 {
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
        .sum()
}
