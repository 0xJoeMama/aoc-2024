use aoc_companion_codegen::day;
use regex::Regex;

#[day(3, parser=parse, part1=part1, part2=part2)]
const DAY: u32 = 3;

type Input = String;

type Output1 = u64;
type Output2 = u64;

fn parse(input: &str) -> Input {
    // join them to a single string
    input.lines().collect::<String>()
}

#[derive(PartialEq)]
enum State {
    Do,
    Dont,
    Prod(u64),
}

fn part1(input: &Input) -> Output1 {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    regex
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [x, y])| x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap())
        .sum()
}

fn part2(input: &Input) -> Output2 {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do)(\(\))|(don't)(\(\))").unwrap();
    regex
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(m, [x, y])| match m {
            "do()" => State::Do,
            "don't()" => State::Dont,
            _ => State::Prod(x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap()),
        })
        .fold((State::Do, 0), |(state, sum), newstate| match newstate {
            State::Do => (newstate, sum),
            State::Dont => (newstate, sum),
            State::Prod(p) => {
                if state == State::Do {
                    (state, sum + p)
                } else {
                    (state, sum)
                }
            }
        })
        .1
}
