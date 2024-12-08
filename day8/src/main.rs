fn main() {
    let input = util::read_input("day8/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

use std::collections::{HashMap, HashSet};

fn part1(data: &str) -> String {
    let mut city = City::parse(data);

    city.find_anti_nodes(false);

    format!("{}", city.anti_nodes.len())
}

fn part2(data: &str) -> String {
    let mut city = City::parse(data);

    city.find_anti_nodes(true);

    format!("{}", city.anti_nodes.len())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn add(&self, other: &Point) -> Point {
        let Point(x1, y1) = self;
        let Point(x2, y2) = other;

        Point(x1 + x2, y1 + y2)
    }

    fn sub(&self, other: &Point) -> Point {
        let Point(x1, y1) = self;
        let Point(x2, y2) = other;

        Point(x1 - x2, y1 - y2)
    }
}

struct City {
    antennas: HashMap<char, Vec<Point>>,
    height: i32,
    width: i32,
    anti_nodes: HashSet<Point>,
}

impl City {
    fn parse(data: &str) -> City {
        let lines = data
            .trim()
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>();

        let height = lines.len() as i32;
        let width = lines[0].len() as i32;

        let mut chars = HashSet::new();

        let mut antennas = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    chars.insert(c);
                    antennas
                        .entry(c)
                        .or_insert(Vec::new())
                        .push(Point(x as i32, y as i32));
                }
            }
        }

        City {
            antennas,
            height,
            width,
            anti_nodes: HashSet::new(),
        }
    }

    fn find_anti_nodes(&mut self, harmonics: bool) {
        let mut anti_nodes = HashSet::new();
        for (_c, points) in self.antennas.iter() {
            for i in 0..points.len() {
                for j in i + 1..points.len() {
                    let p1 = points[i];
                    let p2 = points[j];

                    let delta = p2.sub(&p1);

                    if harmonics {
                        let mut h1 = p1;
                        while self.in_bounds(&h1) {
                            anti_nodes.insert(h1);
                            h1 = h1.sub(&delta);
                        }

                        let mut h2 = p2;
                        while self.in_bounds(&h2) {
                            anti_nodes.insert(h2);
                            h2 = h2.add(&delta);
                        }
                    } else {
                        let h1 = p1.sub(&delta);
                        if self.in_bounds(&h1) {
                            anti_nodes.insert(h1);
                        }
                        let h2 = p2.add(&delta);
                        if self.in_bounds(&h2) {
                            anti_nodes.insert(h2);
                        }
                    }
                }
            }
        }
        self.anti_nodes = anti_nodes
    }

    fn in_bounds(&self, p: &Point) -> bool {
        let Point(x, y) = *p;

        x >= 0 && y >= 0 && x < self.width && y < self.height
    }
}

impl std::fmt::Debug for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = Vec::new();
        for y in 0..self.height {
            let mut row = Vec::new();
            for x in 0..self.width {
                if self.anti_nodes.contains(&Point(x, y)) {
                    row.push('#');
                } else {
                    row.push('.');
                }
            }
            rows.push(row);
        }

        for (c, points) in self.antennas.iter() {
            for Point(x, y) in points {
                rows[*y as usize][*x as usize] = *c;
            }
        }

        for row in rows {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "14");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "34");
    }
}