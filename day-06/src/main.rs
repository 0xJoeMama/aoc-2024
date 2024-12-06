use aoc_companion::point::Point;
use aoc_companion_codegen::day;
use std::collections::{HashMap, HashSet};

#[day(6, parser=parser, part1=part1, part2=part2)]
const DAY: u32 = 6;

#[derive(PartialEq, Clone, Eq)]
enum State {
    Obstacle,
    Free,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Guard {
    pos: Point,
    facing: Point,
}

impl Guard {
    fn new(pos: Point, facing_x: i64, facing_y: i64) -> Guard {
        Guard {
            pos,
            facing: Point::new(facing_x, facing_y, 0),
        }
    }

    fn rotate(self: &mut Self) {
        // basically -90 degrees rotation matrix
        let tmp = self.facing.x;
        self.facing.x = self.facing.y;
        self.facing.y = -tmp;
    }
}

type Input = (HashMap<Point, State>, Guard);

fn parser(input: &str) -> Input {
    let (map, guard) = input.lines().enumerate().fold(
        (HashMap::with_capacity(input.lines().count()), None),
        |(mut map, guard), (i, line)| {
            let mut new_guard = None;
            for (p, state, guard) in line.chars().enumerate().map(|(j, c)| {
                let p = Point::new(i as i64, j as i64, 0);
                let (state, guard) = match c {
                    '.' => (State::Free, None),
                    '#' => (State::Obstacle, None),
                    'v' => (State::Free, Some(Guard::new(p, 1, 0))),
                    '^' => (State::Free, Some(Guard::new(p, -1, 0))),
                    '>' => (State::Free, Some(Guard::new(p, 0, 1))),
                    '<' => (State::Free, Some(Guard::new(p, 0, -1))),
                    _ => unreachable!(),
                };

                (p, state, guard)
            }) {
                map.entry(p).insert_entry(state);
                if guard.is_some() && new_guard.is_none() {
                    let _ = new_guard.insert(guard.unwrap());
                }
            }
            (map, guard.or(new_guard))
        },
    );

    (map, guard.unwrap())
}

fn perform_move(map: impl Fn(&Point) -> Option<State>, mut guard: Guard) -> Option<Guard> {
    let mut new_pos = guard.pos + guard.facing;
    loop {
        if let Some(state) = map(&new_pos) {
            if state == State::Obstacle {
                guard.rotate();
                new_pos = guard.pos + guard.facing;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    map(&new_pos).map(|_| {
        guard.pos = new_pos;
        guard
    })
}

fn part1((map, guard): &Input) -> usize {
    let mut poses = HashSet::new();
    poses.insert(guard.pos);
    let mut guard = guard.clone();

    while let Some(new_guard) = perform_move(|p| map.get(p).cloned(), guard) {
        poses.insert(new_guard.pos);
        guard = new_guard;
    }

    poses.len()
}

fn part2((map, guard): &Input) -> usize {
    let mut cnt = 0;

    let mut attempt_poses: HashSet<Point> = HashSet::new();
    let mut tmp_guard = guard.clone();

    while let Some(new_guard) = perform_move(|p| map.get(p).cloned(), tmp_guard) {
        if map
            .get(&new_guard.pos)
            .is_some_and(|neighbor| *neighbor == State::Free)
            && new_guard.pos != guard.pos
        {
            attempt_poses.insert(new_guard.pos);
        }

        tmp_guard = new_guard;
    }
    println!("Poses : {}", attempt_poses.len());

    let mut visited: HashSet<Guard> = HashSet::with_capacity(map.len());
    'outer: for pos in &attempt_poses {
        visited.clear();

        visited.insert(guard.clone());
        let mut guard = guard.clone();

        while let Some(new_guard) = perform_move(
            |p| {
                if p == pos {
                    Some(State::Obstacle)
                } else {
                    map.get(p).cloned()
                }
            },
            guard,
        ) {
            if !visited.insert(new_guard.clone()) {
                cnt += 1;
                continue 'outer;
            }

            guard = new_guard;
        }
    }

    cnt
}
