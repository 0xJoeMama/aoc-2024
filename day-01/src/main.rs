use std::collections::HashMap;

const FILE: &str = include_str!("../input.txt");

fn main() {
    let (mut left, mut right) = FILE
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace().flat_map(|s| s.parse::<u32>());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect::<(Vec<u32>, Vec<u32>)>();

    left.sort();
    right.sort();

    // part 1
    let sum = left
        .iter()
        .zip(right.iter())
        .map(|(left_n, right_n)| right_n.abs_diff(*left_n))
        .sum::<u32>();

    let freq_map: HashMap<u32, u32> = right.iter().fold(HashMap::new(), |mut acc, v| {
        *acc.entry(*v).or_insert(0) += 1;
        acc
    });

    let new_sum = left
        .iter()
        .filter_map(|left_v| freq_map.get(left_v).map(|it| it * left_v))
        .sum::<u32>();

    println!("{sum}, {new_sum}");
}
