use std::collections::{hash_map::Entry, HashMap};

use aoc_companion_codegen::day;

#[day(9, parser=parser, part1=part1, part2=part2)]
const DAY: u32 = 9;

type Input = Vec<u32>;

fn parser(input: &str) -> Input {
    input
        .lines()
        .flat_map(|it| it.chars())
        .flat_map(|it| it.to_digit(10))
        .collect()
}

fn part1(input: &Input) -> usize {
    let mut idx = 0;
    let mut left_idx = 0; // first element
    let mut left_file = 0;

    let mut right_idx = input.len() - 1; // last element
    let mut right = input[right_idx];
    let mut right_file = right_idx / 2;

    let mut cksum = 0;
    while left_idx < right_idx {
        let left_value = input[left_idx];
        // handle the already full block
        for _ in 0..left_value {
            cksum += left_file * idx;
            idx += 1;
        }
        left_file += 1;

        // handle picking up stuff from the end
        let mut right_to_take = input[left_idx + 1];
        while right_to_take > 0 {
            if right == 0 {
                right_idx -= 2;
                right = input[right_idx];
                right_file -= 1;
            }

            cksum += right_file * idx;
            idx += 1;
            right -= 1;
            right_to_take -= 1;
        }

        left_idx += 2;
    }

    for _ in 0..right {
        cksum += idx * left_file;
        idx += 1;
    }

    cksum
}

struct Bucket {
    curr_cap: u32,
    files: Vec<usize>,
}

fn part2(input: &Input) -> usize {
    let mut right_idx = input.len();

    let mut consume_map: HashMap<usize, Bucket> = HashMap::with_capacity(input.len() / 2);

    while right_idx > 0 {
        // get all empty positions from the beginning
        for i in (1..right_idx).step_by(2) {
            let bucket = consume_map.entry(i).or_insert(Bucket {
                curr_cap: input[i],
                files: Vec::new(),
            });
            if bucket.curr_cap > input[right_idx] {
                bucket.files.push(right_idx);
                bucket.curr_cap -= input[right_idx];
            }
        }

        right_idx -= 2;
    }

    todo!()
}
