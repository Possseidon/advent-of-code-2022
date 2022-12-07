use itertools::Itertools;

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|rucksack| {
            let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);
            item_priority(
                compartment1
                    .chars()
                    .find(|&item| compartment2.contains(item))
                    .unwrap(),
            )
        })
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let (rucksack1, rucksack2, rucksack3) = group.next_tuple().unwrap();
            item_priority(
                rucksack1
                    .chars()
                    .find(|&item| rucksack2.contains(item) && rucksack3.contains(item))
                    .unwrap(),
            )
        })
        .sum::<u32>()
        .to_string()
}

fn item_priority(item: char) -> u32 {
    let item = item.try_into().unwrap();
    match item {
        b'a'..=b'z' => 1 + item - b'a',
        b'A'..=b'Z' => 27 + item - b'A',
        _ => panic!("invalid item"),
    }
    .into()
}
