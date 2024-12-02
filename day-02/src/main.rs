use std::{cmp::Ordering, usize};

use aoc_companion_codegen::day;

#[day(2, parser=parser, part1=part1, part2=part2)]
const DAY: u32 = 2;

fn parser(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|v| v.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

fn is_safe(line: &[i32]) -> bool {
    let ordering = line[0].cmp(&line[1]);
    if ordering == Ordering::Equal {
        return false;
    }

    line.windows(2)
        .all(|v| v[0].cmp(&v[1]) == ordering && (1..=3).contains(&v[0].abs_diff(v[1])))
}

fn part1(input: &[Vec<i32>]) -> usize {
    input.iter().filter(|line| is_safe(line)).count()
}

fn part2(input: &[Vec<i32>]) -> usize {
    input
        .iter()
        .filter(|line| {
            is_safe(line)
                || (0..line.len()).any(|i| {
                    // split_at excludes on the first slice and included on the second one
                    let (a, b) = line.split_at(i);
                    let modified_line = a
                        .iter()
                        .chain(b.iter().skip(1)) // skip here ignores the ignored element
                        .copied()
                        .collect::<Vec<_>>(); // there is definititely a way to not copy here do this

                    is_safe(&modified_line)
                })
        })
        .count()
}
