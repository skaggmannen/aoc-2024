use std::collections::{BinaryHeap, HashMap, HashSet};


fn main() {
    let input = util::read_input("day18/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let problem = Problem::from(input);

    if let Some(result) = problem.solve(71, 1024) {
        format!("{}", result)
    } else {
        format!("this is impossible!")
    }
}

fn part2(input: &str) -> String {
    let problem = Problem::from(input);

    for rounds in 1..problem.bytes.len() {
        let result = problem.solve(71, rounds);
        let (x, y) = problem.bytes[rounds-1];
        if result.is_none() {
            return format!("{},{}", x, y);
        }
    }

    format!("this is impossible!")
}

struct Problem {
    bytes: Vec<(i32, i32)>,
}

impl Problem {
    fn from(data: &str) -> Self {
        let bytes = util::to_lines(data).iter().map(|l| {
            let (x, y) = l.split_once(",").unwrap();

            (x.parse().unwrap(), y.parse().unwrap())
        }).collect();

        Problem {
            bytes,
        }
    }

    fn solve(&self, size: i32, rounds: usize) -> Option<usize> {
        let mut corrupted = HashSet::new();

        for i in 0..rounds {
            corrupted.insert(self.bytes[i]);
        }

        let mut start = Path{
            size,
            pos: (0, 0),
            steps: 0,
        };
        start.visit((0, 0));
    
        let mut paths = BinaryHeap::new();
        paths.push(start);

        let mut visited = HashMap::new();
        while let Some(path) = paths.pop() {
            if let Some(&steps) = visited.get(&path.pos) {
                if steps <= path.steps {
                    continue;
                }
            }
            visited.insert(path.pos, path.steps);

            if path.pos == (size-1, size-1) {
                return Some(path.steps - 1);
            }

            if let Some(next) = path.down() {
                if !corrupted.contains(&next.pos) {
                    paths.push(next);
                }
            }
            if let Some(next) = path.left() {
                if !corrupted.contains(&next.pos) {
                    paths.push(next);
                }
            }
            if let Some(next) = path.right() {
                if !corrupted.contains(&next.pos) {
                    paths.push(next);
                }
            }
            if let Some(next) = path.up() {
                if !corrupted.contains(&next.pos) {
                    paths.push(next);
                }
            }
        }

        None
    }
}

#[derive(Clone,PartialEq,Eq)]
struct Path {
    size: i32,
    pos: (i32, i32),
    steps: usize,
}

impl Path {
    fn left(&self) -> Option<Path> {
        self.go((-1, 0))
    }

    fn right(&self) -> Option<Path> {
        self.go((1, 0))
    }

    fn up(&self) -> Option<Path> {
        self.go((0, -1))
    }

    fn down(&self) -> Option<Path> {
        self.go((0, 1))
    }

    fn go(&self, d: (i32, i32)) -> Option<Path> {
        let (mut x, mut y) = self.pos;
        let (dx, dy) = d;
        
        x += dx;
        y += dy;

        let pos = (x, y);

        if x < 0 || x >= self.size || y < 0 || y >= self.size {
            None
        } else {
            let mut path = self.clone();
            path.visit(pos);

            Some(path)
        }
    }

    fn visit(&mut self, (x, y): (i32, i32)) {
        self.pos = (x, y);
        self.steps += 1;
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return other.steps.partial_cmp(&self.steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    ";

    #[test]
    fn test_part1() {
        let problem = Problem::from(INPUT);
    
        assert_eq!(problem.solve(7, 12), Some(22));
    }


    #[test]
    fn test_part1_real() {
        assert_eq!(part1(util::read_input("input.txt").as_str()), "324");
    }

    #[test]
    fn test_part2() {
        let problem = Problem::from(INPUT);
    
        let mut result = None;
        for rounds in 1..problem.bytes.len() {
            if problem.solve(7, rounds).is_none() {
                result = Some(problem.bytes[rounds-1]);
                break;
            }
        };

        if let Some(pos) = result {
            assert_eq!(pos, (6,1));
        } else {
            panic!("this is impossible!");
        }
    }
}
