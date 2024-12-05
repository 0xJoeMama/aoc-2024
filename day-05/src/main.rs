use aoc_companion_codegen::day;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[day(5, parser=parser, part1=part1, part2=part2)]
const DAY: u32 = 5;

type Input = (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>);

fn parser(input: &str) -> Input {
    let (order, updates) = input.split_once("\n\n").unwrap();

    let updates = updates
        .lines()
        .map(|it| it.split(",").flat_map(|num| num.parse::<u32>()).collect())
        .filter(|it: &Vec<u32>| !it.is_empty())
        .collect::<Vec<Vec<u32>>>();

    let relations = order
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("|").unwrap();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>()
        .iter()
        .fold(HashMap::new(), |mut acc, (a, b)| {
            acc.entry(*a).or_insert(HashSet::new()).insert(*b);
            acc
        });

    (relations, updates)
}

fn order_by_rel(a: &u32, b: &u32, relations: &HashMap<u32, HashSet<u32>>) -> Ordering {
    if relations
        .get(a)
        .map(|targets| targets.contains(b))
        .unwrap_or(false)
    {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn do_part((relations, updates): &Input, cmp: impl Fn(&Vec<u32>, &Vec<u32>) -> bool) -> u32 {
    updates
        .iter()
        .filter_map(|update| {
            let mut local_update = update.clone();
            local_update.sort_by(|a, b| order_by_rel(a, b, relations));

            if cmp(&local_update, update) {
                Some(local_update)
            } else {
                None
            }
        })
        .map(|new| new[new.len() / 2])
        .sum()
}

fn part1(input: &Input) -> u32 {
    do_part(input, |a, b| a == b)
}

fn part2(input: &Input) -> u32 {
    do_part(input, |a, b| a != b)
}
