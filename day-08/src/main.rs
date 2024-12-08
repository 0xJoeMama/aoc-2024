use std::collections::{HashMap, HashSet};

use aoc_companion::point::Point;
use aoc_companion_codegen::day;

#[day(8, parser=parser, part1=part1, part2=part2)]
const DAY: u32 = 8;

type Input = ((usize, usize), HashMap<char, Vec<Point>>);

fn is_within_bounds(it: &Point, bounds: &(usize, usize)) -> bool {
    it.x >= 0 && it.y >= 0 && it.x <= bounds.0 as i64 && it.y <= bounds.1 as i64
}

fn parser(input: &str) -> Input {
    let mut antennas = HashMap::with_capacity(256);
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
            antennas
                .entry(c)
                .or_insert(Vec::with_capacity(10))
                .push(Point::new(i as i64, j as i64, 0));
        }
    }
    let max_x = input.lines().count() - 1;
    let max_y = input.lines().next().unwrap().chars().count() - 1;

    ((max_x, max_y), antennas)
}

fn part1((bounds, input): &Input) -> usize {
    let mut antinode_poses = HashSet::with_capacity(1000);
    for antennas in input.values() {
        for a1 in antennas {
            for a2 in antennas {
                if a2 == a1 {
                    continue;
                }

                let diff = *a2 - *a1;
                let antinode1 = *a1 - diff;
                let antinode2 = *a2 + diff;

                if is_within_bounds(&antinode1, bounds) {
                    antinode_poses.insert(antinode1);
                }
                if is_within_bounds(&antinode2, bounds) {
                    antinode_poses.insert(antinode2);
                }
            }
        }
    }
    antinode_poses.len()
}

fn part2((bounds, input): &Input) -> usize {
    let mut antinode_poses = HashSet::with_capacity(1000);
    for antennas in input.values() {
        for a1 in antennas {
            for a2 in antennas {
                if a2 == a1 {
                    continue;
                }

                let diff = *a2 - *a1;

                let mut antinode1 = *a1;
                while is_within_bounds(&antinode1, bounds) {
                    antinode_poses.insert(antinode1);
                    antinode1 = antinode1 - diff;
                }

                let mut antinode2 = *a2;
                while is_within_bounds(&antinode2, bounds) {
                    antinode_poses.insert(antinode2);
                    antinode2 = antinode2 - diff;
                }
            }
        }
    }
    antinode_poses.len()
}
