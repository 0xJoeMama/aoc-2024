use std::{str::FromStr, usize};

use aoc_companion::point::{Point, PointsBetween};
use aoc_companion_codegen::day;

#[day(4, parser=parse, part1=part1, part2=part2)]
const DAY: u32 = 4;

type Input = Vec<String>;
type Output1 = usize;
type Output2 = usize;

fn parse(input: &str) -> Input {
    input.lines().flat_map(String::from_str).collect()
}

const XMAS: &str = "XMAS";

const OFFSETS: &[Point; 8] = &[
    Point::new(0, 3, 0),
    Point::new(0, -3, 0),
    Point::new(3, 0, 0),
    Point::new(-3, 0, 0),
    Point::new(-3, 3, 0),
    Point::new(3, -3, 0),
    Point::new(3, 3, 0),
    Point::new(-3, -3, 0),
];

fn look_around(x: usize, y: usize, input: &Input) -> usize {
    let loc = Point::new(x as i64, y as i64, 0);
    OFFSETS
        .iter()
        .filter(|offset| {
            let origin = loc + *offset;
            PointsBetween::with_step(loc, origin, **offset / 3)
                .enumerate()
                .all(|(i, p)| {
                    let char_input = input
                        .get(p.x as usize)
                        .and_then(|line| line.chars().nth(p.y as usize));

                    char_input.is_some_and(|c| {
                        XMAS.chars()
                            .nth(i)
                            .map(|xmas_c| xmas_c == c)
                            .unwrap_or(false)
                    })
                })
        })
        .count()
}

const MAS_OFFSETS: &[Point; 4] = &[
    Point::new(-1, -1, 0),
    Point::new(1, 1, 0),
    Point::new(1, -1, 0),
    Point::new(-1, 1, 0),
];

fn same_line_occurences(ps: &[(Point, char)], niddle: char) -> usize {
    let (p_target, niddle) = if let Some(s) = ps.iter().find(|(_, c)| *c == niddle) {
        s
    } else {
        return 0;
    };

    ps.iter()
        .filter(|(p, c)| c == niddle && (p_target.x == p.x || p_target.y == p.y))
        .count()
}

fn is_x_mas(x: usize, y: usize, input: &Input) -> bool {
    let loc = Point::new(x as i64, y as i64, 0);
    let ps: Vec<(Point, char)> = MAS_OFFSETS
        .iter()
        .map(|it| *it + loc)
        .filter_map(|p| {
            input
                .get(p.x as usize)
                .and_then(|line| line.chars().nth(p.y as usize))
                .map(|it| (p, it))
        })
        .collect();

    if ps.len() != 4 {
        return false;
    }

    same_line_occurences(&ps, 'S') == 2 && same_line_occurences(&ps, 'M') == 2
}

fn part1(input: &Input) -> Output1 {
    input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'X' {
                        look_around(i, j, input)
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn part2(input: &Input) -> Output2 {
    input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(j, c)| *c == 'A' && is_x_mas(i, *j, input))
                .count()
        })
        .sum()
}
