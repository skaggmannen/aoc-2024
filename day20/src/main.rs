use std::collections::{HashMap, HashSet};

fn main() {
    let input = util::read_input("day20/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let problem = Problem::from(input);

    format!("{}", problem.solve(2, |v| v >= 100))
}

fn part2(input: &str) -> String {
    let problem = Problem::from(input);
    
    format!("{}",problem.solve(20, |v| v >= 100))
}

struct Problem {
    walls: HashSet<(i32, i32)>,
    start: (i32, i32),
    end: (i32, i32),
}

impl Problem {
    fn from(data: &str) -> Self {
        let lines = util::to_lines(data);

        let mut walls = HashSet::new();
        let mut start = None;
        let mut end = None;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = (x as i32, y as i32);

                if c == '#' {
                    walls.insert(pos);
                } else if c == 'S' {
                    start = Some(pos);
                } else if c == 'E' {
                    end = Some(pos);
                }
            }

        }
        Problem {
            walls,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn solve<P: Fn(usize) -> bool>(&self, cheat_size: i32, p: P) -> usize {
        let race = self.find_path();
        let positions = race.iter().enumerate().map(|(i, &pos)| {
            (pos, i)
        }).collect::<HashMap<(i32, i32), usize>>();

        let mut cheats = HashSet::new();
        for dx in 0..=cheat_size {
            for dy in 0..=(cheat_size-dx) {
                cheats.insert((dx, dy));
                cheats.insert((-dx, dy));
                cheats.insert((dx, -dy));
                cheats.insert((-dx, -dy));
            }
        }

        let mut count = 0;
        for (i, (x, y)) in race.iter().enumerate() {
            // Let's see how far ahead we get if we cheat here.

            for &(dx, dy) in cheats.iter() {
                let next = (x+dx, y+dy);
                if let Some(&next_i) = positions.get(&next) {
                    if next_i < i {
                        continue;
                    }

                    let gain = next_i - i - (dx.abs() as usize) - (dy.abs() as usize);
                    if p(gain) {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }

    fn find_path(&self) -> Vec<(i32, i32)> {
        let mut path = Vec::new();

        let mut pos = self.start;
        let mut visited = HashSet::new();
        'outer: while pos != self.end {
            visited.insert(pos);

            let (x, y) = pos;
            for (dx, dy) in [
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
            ] {
                let next = (x+dx, y+dy);
                if !visited.contains(&next) && !self.walls.contains(&next) {
                    path.push(pos);
                    pos = next;
                    continue 'outer;
                }
            }
        }
        path.push(pos);

        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    ";

    #[test]
    fn test_part1_ex1() {
        let problem = Problem::from(INPUT);
        assert_eq!(problem.find_path().len(), 85);
        assert_eq!(problem.solve(2, |v| v == 2), 14);
        assert_eq!(problem.solve(2, |v| v == 4), 14);
        assert_eq!(problem.solve(2, |v| v == 6), 2);
        assert_eq!(problem.solve(2, |v| v == 8), 4);
        assert_eq!(problem.solve(2, |v| v == 10), 2);
        assert_eq!(problem.solve(2, |v| v == 12), 3);
        assert_eq!(problem.solve(2, |v| v == 20), 1);
        assert_eq!(problem.solve(2, |v| v == 36), 1);
        assert_eq!(problem.solve(2, |v| v == 38), 1);
        assert_eq!(problem.solve(2, |v| v == 40), 1);
        assert_eq!(problem.solve(2, |v| v == 64), 1);
    }

    #[test]
    fn test_part2() {
        let problem = Problem::from(INPUT);
        assert_eq!(problem.find_path().len(), 85);
        assert_eq!(problem.solve(20, |v| v == 50), 32);
        assert_eq!(problem.solve(20, |v| v == 52), 31);
        assert_eq!(problem.solve(20, |v| v == 54), 29);
        assert_eq!(problem.solve(20, |v| v == 56), 39);
        assert_eq!(problem.solve(20, |v| v == 58), 25);
        assert_eq!(problem.solve(20, |v| v == 60), 23);
        assert_eq!(problem.solve(20, |v| v == 62), 20);
        assert_eq!(problem.solve(20, |v| v == 64), 19);
        assert_eq!(problem.solve(20, |v| v == 66), 12);
        assert_eq!(problem.solve(20, |v| v == 68), 14);
        assert_eq!(problem.solve(20, |v| v == 70), 12);
        assert_eq!(problem.solve(20, |v| v == 72), 22);
        assert_eq!(problem.solve(20, |v| v == 74), 4);
        assert_eq!(problem.solve(20, |v| v == 76), 3);
    }
}