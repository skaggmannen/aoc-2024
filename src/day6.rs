use std::collections::HashMap;

pub fn part1(data: &str) -> String {
    let mut map = parse_input(data);

    map.run();
    
    format!("{}", map.guard_route.len())
}

pub fn part2(data: &str) -> String {
    let original = parse_input(data);

    let mut first = original.clone();
    first.run();

    let mut result = 0;
    for &(x, y) in first.guard_route.keys() {
        if (x, y) == original.guard {
            continue;
        } else if original.obstructions.contains_key(&(x, y)) {
            continue;
        }

        let mut map = original.clone();
        map.obstructions.insert((x, y), 'O');

        if map.run() {
            result += 1;
        }
    }

    format!("{}", result)
}

#[derive(Clone)]
struct Map {
    width: i32,
    height: i32,
    obstructions: HashMap<(i32, i32), char>,
    guard: (i32, i32),
    guard_route: HashMap<(i32, i32), (i32, i32)>,
}

impl Map {
    fn run(&mut self) -> bool {
        let (mut dx, mut dy) = (0, -1);

        loop {
            let (mut x, mut y) = self.guard;
            if x < 0 || x >= self.width || y < 0 || y >= self.height {
                return false;
            } else if let Some(direction) = self.guard_route.get(&(x, y)) {
                if *direction == (dx, dy) {
                    return true;
                }
            }

            if self.obstructions.contains_key(&(x, y)) {
                // Back up
                (x, y) = (x - dx, y - dy);

                // Turn right
                (dx, dy) = match (dx, dy) {
                    (0, -1) => (1, 0),
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    _ => panic!("Invalid direction!"),
                };
            } else {
                self.guard_route.insert((x, y), (dx, dy));
            }
            
            // Move forward
            self.guard = (x + dx, y + dy);
        }
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(c) = self.obstructions.get(&(x, y)) {
                    write!(f, "{}", c)?;
                } else if (x, y) == self.guard {
                    write!(f, "^")?;
                } else if self.guard_route.contains_key(&(x, y)) {
                    write!(f, "X")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "{}", self.guard_route.len())?;

        Ok(())
    }
}

fn parse_input(data: &str) -> Map {
    let lines = data.trim().lines().map(|line| line.trim()).collect::<Vec<_>>();

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut obstructions = HashMap::new();
    let mut guard = (0, 0);

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            match c {
                '#' => {
                    obstructions.insert(pos, '#');
                }
                '^' => {
                    guard = pos;
                }
                _ => (),
            };
        }
    }

    Map {
        width,
        height,
        obstructions,
        guard,
        guard_route: HashMap::new(),
    }
}

fn is_valid(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
    for i in 0..update.len() {
        let key = update[i];

        if let Some(rules) = rules.get(&key) {
            for page in rules {
                if update[0..i].contains(page) {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "41");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "6");
    }
}
