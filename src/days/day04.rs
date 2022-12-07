use std::ops::RangeInclusive;

pub fn part1(input: String) -> String {
    input
        .lines()
        .filter(|sections| {
            let (range1, range2) = split_and_parse_sections(sections);
            let range_1_in_2 = range1.contains(range2.start()) && range1.contains(range2.end());
            let range_2_in_1 = range2.contains(range1.start()) && range2.contains(range1.end());
            range_1_in_2 || range_2_in_1
        })
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .filter(|sections| {
            let (range1, range2) = split_and_parse_sections(sections);
            range1.start() <= range2.end() && range2.start() <= range1.end()
        })
        .count()
        .to_string()
}

fn split_and_parse_sections(sections: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let (range1, range2) = split_sections(sections);
    (parse_sections(range1), parse_sections(range2))
}

fn split_sections(sections: &str) -> (&str, &str) {
    sections.split_once(',').unwrap()
}

fn parse_sections(range: &str) -> RangeInclusive<u32> {
    let (from, to) = range.split_once('-').unwrap();
    from.parse().unwrap()..=to.parse().unwrap()
}
