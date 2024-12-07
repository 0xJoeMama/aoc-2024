use aoc_companion_codegen::day;

#[day(7, parser=parser, part1=part1, part2=part2)]
const DAY: u32 = 7;

type Input = Vec<(i64, Vec<i64>)>;

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Concat,
}

struct OperIterator {
    max_ops: usize,
    curr_val: Option<u32>,
    modulus: u32,
}

impl OperIterator {
    fn new(vals: &[i64], modulus: u32) -> Self {
        Self {
            max_ops: vals.len() - 1,
            curr_val: None,
            modulus,
        }
    }
}

struct OpIterator {
    val: u32,
    max_idx: usize,
    idx: usize,
    modulus: u32,
}

impl OpIterator {
    fn with_modulus(val: u32, max_idx: usize, modulus: u32) -> Self {
        let idx = 0;

        Self {
            idx,
            max_idx,
            val,
            modulus,
        }
    }
}

impl Iterator for OpIterator {
    type Item = Op;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx + 1 > self.max_idx {
            return None;
        }

        let selected_digit = (self.val / self.modulus.pow(self.idx as u32)) % self.modulus;
        let res = if selected_digit == 0 {
            Some(Op::Add)
        } else if selected_digit == 1 {
            Some(Op::Mul)
        } else if selected_digit == 2 {
            Some(Op::Concat)
        } else {
            todo!("up to base 3 allowed")
        };

        self.idx += 1;
        res
    }
}

impl Iterator for OperIterator {
    type Item = OpIterator;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_val {
            Some(val) => {
                let val = val + 1; // create the new val
                let non_zero_digits = {
                    let mut digits = 0;
                    let mut val = val;
                    while val != 0 {
                        digits += 1;
                        val /= self.modulus;
                    }

                    digits
                };

                if non_zero_digits > self.max_ops as u32 {
                    None
                } else {
                    self.curr_val = Some(val);
                    Some(OpIterator::with_modulus(val, self.max_ops, self.modulus))
                }
            }
            None => {
                self.curr_val = Some(0);
                Some(OpIterator::with_modulus(0, self.max_ops, self.modulus))
            }
        }
    }
}

fn parser(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (test, values) = line.split_once(":").expect("invalid line");
            (
                test.parse().expect("integer cannot be parsed"),
                values.split(' ').flat_map(|v| v.parse()).collect(),
            )
        })
        .collect()
}

fn do_part(input: &Input, modulus: u32) -> i64 {
    input
        .iter()
        .filter(|(test_case, values)| {
            OperIterator::new(values, modulus).any(|op_iterator| {
                op_iterator
                    .zip(values.iter().skip(1))
                    // notice that this actually double checks a lot of values
                    // if i had used recursion this wouldn't have happened
                    .try_fold(values[0], |old_val, (op, curr)| {
                        let res = match op {
                            Op::Add => old_val + curr,
                            Op::Mul => old_val * curr,
                            Op::Concat => {
                                let mut digits = 0;
                                let mut curr_cp = *curr;
                                while curr_cp != 0 {
                                    digits += 1;
                                    curr_cp /= 10;
                                }

                                let res = old_val * 10i64.pow(digits) + curr;
                                res
                            }
                        };

                        if res <= *test_case {
                            Some(res)
                        } else {
                            None
                        }
                    })
                    .is_some_and(|it| it == *test_case)
            })
        })
        .map(|(test_case, _)| test_case)
        .sum()
}

fn part1(input: &Input) -> i64 {
    do_part(input, 2)
}

fn part2(input: &Input) -> i64 {
    do_part(input, 3)
}
