use std::{collections::{HashMap, HashSet}, fmt::Debug};

fn main() {
    let input = util::read_input("day14/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut problem = Problem::from(input);

    problem.run(100);

    format!("{}", problem.solve())
}

fn part2(input: &str) -> String {
    let mut problem = Problem::from(input);

    problem.run_until_repeat();

    format!("Find the christams tree above 👆")
}

struct Problem {
    width: i32,
    height: i32,
    robots: Vec<Robot>,
}

impl From<&str> for Problem {
    fn from(data: &str) -> Self {
        let robots = data
            .lines()
            .filter_map(|s| util::trim_space(s))
            .map(|s| Robot::parse(s))
            .collect();

        Problem { width: 101, height: 103, robots }
    }
}

impl Problem {
    fn run(&mut self, iterations: usize) {
        for _ in 0..iterations {
            for r in self.robots.iter_mut() {
                r.travel(self.width, self.height);
            }
        }
    }

    fn run_until_repeat(&mut self) {
        let mut history = HashSet::new();

        let mut i = 0;
        loop {
            if history.contains(&self.robots) {
                break;
            }
            history.insert(self.robots.clone());

            if i % self.width == 4 {
                println!("Iteration {}:", i);
                println!("{:?}", self);
            }
            i += 1;

            // 4, 76, 105, 206

            for r in self.robots.iter_mut() {
                r.travel(self.width, self.height);
            }
        }
    }

    fn solve(&mut self) -> i32 {
        let mut quadrants = (0, 0, 0, 0);
        for r in self.robots.iter() {
            let (x, y) = r.p;

            if x < self.width/2 && y < self.height/2 {
                quadrants.0 += 1;
            } else if x > self.width/2 && y < self.height/2 {
                quadrants.1 += 1;
            } else if x > self.width/2 && y > self.height/2 {
                quadrants.2 += 1;
            } else if x < self.width/2 && y > self.height/2 {
                quadrants.3 += 1;
            }
        }

        quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
    }
}

impl Debug for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut counts = HashMap::new();

        for r in self.robots.iter() {
            *counts.entry(r.p).or_insert(0) += 1
        }

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(_) = counts.get(&(x, y)) {
                    write!(f, "X")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    fn parse(s: &str) -> Self {
        let (p, v) = s.split_once(" ").unwrap();

        let (p_x, p_y) = p.strip_prefix("p=").unwrap().split_once(",").unwrap();
        let (v_x, v_y) = v.strip_prefix("v=").unwrap().split_once(",").unwrap();

        Robot {
            p: (p_x.parse().unwrap(), p_y.parse().unwrap()),
            v: (v_x.parse().unwrap(), v_y.parse().unwrap()),
        }
    }

    fn travel(&mut self, width: i32, height: i32) {
        let (x, y) = self.p;
        let (v_x, v_y) = self.v;

        let (mut x, mut y) = ((x + v_x), (y + v_y));
        if x < 0 {
            x += width;
        } else if x >= width {
            x -= width;
        }
        if y < 0 {
            y += height;
        } else if y >= height {
            y -= height;
        }

        self.p = (x, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    ";

    #[test]
    fn test_part1() {
        let mut problem = Problem::from(INPUT);
        problem.width = 11;
        problem.height = 7;

        problem.run(100);

        assert_eq!(problem.solve(), 12);
    }

    #[test]
    fn test_travel() {
        let mut r = Robot{
            p: (2, 4),
            v: (2, -3),
        };
        let (width, height) = (11, 7);

        for _ in 0..5 {
            r.travel(width, height);
            println!("{:?}", Problem{ robots: vec![r], width, height});
        }
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "0");
    }
}
