use std::collections::HashMap;

use aoc_companion_codegen::day;
#[day(1, parser=parser, part1=part1, part2=part2)]
const DAY: u32 = 1;

fn parser(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut left, mut right) = input
        .lines()
        .map(|l| {
            aoc_companion::regex_parser!("[0-9]+  [0-9]+"; l => left, right);
            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .collect::<(Vec<_>, Vec<_>)>();

    left.sort();
    right.sort();
    (left, right)
}

fn part1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    left.iter()
        .zip(right.iter())
        .map(|(left_n, right_n)| right_n.abs_diff(*left_n))
        .sum::<u32>()
}

fn part2((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let freq_map: HashMap<u32, u32> =
        right
            .iter()
            .fold(HashMap::with_capacity(right.len()), |mut acc, v| {
                acc.entry(*v).and_modify(|v| *v += 1).or_insert(1);
                acc
            });

    left.iter()
        .filter_map(|left_v| freq_map.get(left_v).map(|it| it * left_v))
        .sum::<u32>()
}
