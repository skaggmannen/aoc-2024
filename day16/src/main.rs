use std::{collections::{BinaryHeap, HashMap, HashSet}, fmt::Debug, io::{stdout, Write}};


fn main() {
    let input = util::read_input("day16/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let map = Map::parse(input);

    format!("{}", map.solve())
}

fn part2(input: &str) -> String {
    let map = Map::parse(input);
    
    format!("{}", map.solve_part2())
}

struct Map {
    size: (i32, i32),
    walls: HashSet<(i32, i32)>,
    start: (i32, i32),
    end: (i32, i32),
}

impl Map {
    fn parse(data: &str) -> Self {
        let lines = util::to_lines(data);

        let mut walls = HashSet::new();
        let mut start = None;
        let mut end = None;

        for y in 0..lines.len() {
            let line: Vec<char> = lines[y].chars().collect();
            for x in 0..line.len() {
                let pos = (x as i32, y as i32);
                match line[x] {
                    '#' => { walls.insert(pos); },
                    'S' => { start = Some(pos); }
                    'E' => { end = Some(pos); },
                    _ => {},
                }
            }
        }

        Map {
            size: (lines[0].len() as i32, lines.len() as i32),
            walls,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn solve(&self) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        heap.push(Reindeer{
            pos: self.start,
            d: (1, 0),
            score: 0,
            steps: HashMap::new(),
        });

        while let Some(reindeer) = heap.pop() {
            if self.walls.contains(&reindeer.pos) {
                continue;
            }
            if visited.contains(&reindeer.pos) {
                continue;
            }
            visited.insert(reindeer.pos);

            if reindeer.pos == self.end {
                heap.push(reindeer);
                break;
            }

            if let Some(straight) = reindeer.straight() {
                heap.push(straight);
            }

            if let Some(left) = reindeer.left() {
                heap.push(left);
            }

            if let Some(right) = reindeer.right() {
                heap.push(right);
            }
        }

        let winner = heap.pop().unwrap();

        self.print(&winner);

        winner.score
    }

    fn solve_part2(&self) -> usize {
        let mut heap = BinaryHeap::new();
        let mut winners: Vec<Reindeer> = Vec::new();
        let mut visited: HashMap<((i32, i32), (i32, i32)), i32> = HashMap::new();

        heap.push(Reindeer{
            pos: self.start,
            d: (1, 0),
            score: 0,
            steps: HashMap::new(),
        });

        while let Some(reindeer) = heap.pop() {
            if self.walls.contains(&reindeer.pos) {
                continue;
            }
            if let Some(prev) = winners.first() {
                if reindeer.score > prev.score {
                    // This reindeer will never win.
                    continue;
                }
            }
            if let Some(&score) = visited.get(&(reindeer.pos, reindeer.d)) {
                if reindeer.score > score {
                    continue;
                }
            }

            visited.insert((reindeer.pos, reindeer.d), reindeer.score);

            if reindeer.pos == self.end {
                if let Some(prev) = winners.first() {
                    if reindeer.score == prev.score {
                        winners.push(reindeer);
                    } else if reindeer.score < prev.score {
                        winners = vec![reindeer];
                    }
                } else {
                    winners = vec![reindeer];
                }
                
                continue;
            }

            
            if let Some(straight) = reindeer.straight() {
                heap.push(straight);
            }

            if let Some(left) = reindeer.left() {
                heap.push(left);
            }

            if let Some(right) = reindeer.right() {
                heap.push(right);
            }
        }

        let mut nice_places = HashSet::new();

        for winner in winners.iter() {
            for &pos in winner.steps.keys() {
                nice_places.insert(pos);
            }
        }

        nice_places.len() + 1 // Also include the start area
    }

    fn print(&self, reindeer: &Reindeer) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let pos = (x, y);
                if self.walls.contains(&pos) {
                    print!("#");
                } else if pos == self.start {
                    print!("S");
                } else if pos == self.end {
                    print!("E");
                } else if let Some(d) = reindeer.steps.get(&pos) {
                    match d {
                        (0, -1) => print!("^"),
                        (1, 0) => print!(">"),
                        (0, 1) => print!("v"),
                        (-1, 0) => print!("<"),
                        _ => print!("?"),
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();

        stdout().flush().unwrap();
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let pos = (x, y);
                if self.walls.contains(&pos) {
                    write!(f, "#")?;
                } else if pos == self.start {
                    write!(f, "S")?;
                } else if pos == self.end {
                    write!(f, "E")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(PartialEq, Eq)]
struct Reindeer {
    pos: (i32, i32),
    d: (i32, i32),
    score: i32,
    steps: HashMap<(i32, i32), (i32, i32)>,
}

impl Reindeer {
    fn straight(&self) -> Option<Reindeer> {
        let (x, y) = self.pos;
        let (dx, dy) = self.d;
        let pos = (x+dx, y+dy);
        let score = self.score + 1;

        if self.steps.contains_key(&pos) {
            return None;
        }

        let mut steps = self.steps.clone();
        steps.insert(pos, (dx, dy));

        Some(Reindeer {
            pos,
            d: (dx, dy),
            score,
            steps,
        })
    }

    fn left(&self) -> Option<Reindeer> {
        let (x, y) = self.pos;
        let (dx, dy) = match self.d {
            (0, -1) => (-1, 0),
            (1, 0) => (0, -1),
            (0, 1) => (1, 0),
            (-1, 0) => (0, 1),
            _ => panic!("invalid direction"),
        };
        let pos =(x+dx, y+dy);
        let score = self.score + 1000 + 1;

        
        if self.steps.contains_key(&pos) {
            return None;
        }

        let mut steps = self.steps.clone();
        steps.insert(pos, (dx, dy));

        Some(Reindeer {
            pos,
            d: (dx, dy),
            score,
            steps,
        })
    }


    fn right(&self) -> Option<Reindeer> {
        let (x, y) = self.pos;
        let (dx, dy) = match self.d {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => panic!("invalid direction"),
        };
        let pos =(x+dx, y+dy);
        let score = self.score + 1000 + 1;

        if self.steps.contains_key(&pos) {
            return None;
        }

        let mut steps = self.steps.clone();
        steps.insert(pos, (dx, dy));

        Some(Reindeer {
            pos,
            d: (dx, dy),
            score,
            steps,
        })
    }
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.score.partial_cmp(&self.score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "7036");
    }

    #[test]
    fn test_part2_ex1() {
        assert_eq!(part2(INPUT), "45");
    }


    #[test]
    fn test_part2_ex2() {
        const INPUT: &str = "
            #################
            #...#...#...#..E#
            #.#.#.#.#.#.#.#.#
            #.#.#.#...#...#.#
            #.#.#.#.###.#.#.#
            #...#.#.#.....#.#
            #.#.#.#.#.#####.#
            #.#...#.#.#.....#
            #.#.#####.#.###.#
            #.#.#.......#...#
            #.#.###.#####.###
            #.#.#...#.....#.#
            #.#.#.#####.###.#
            #.#.#.........#.#
            #.#.#.#########.#
            #S#.............#
            #################
        ";
        assert_eq!(part2(INPUT), "64");
    }
}
