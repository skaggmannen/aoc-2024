
fn main() {
    let input = util::read_input("day25/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let problem = Problem::from(input);

    format!("{}",problem.solve())
}

fn part2(input: &str) -> String {
    let problem = Problem::from(input);
    
    format!("{}",problem.solve())
}

struct Problem {
    keys: Vec<Vec<usize>>,
    locks: Vec<Vec<usize>>,
}

impl Problem {
    fn from(data: &str) -> Self {
        let shapes: Vec<_> = data.trim().split("\n\n").collect();

        let mut keys = Vec::new();
        let mut locks = Vec::new();

        for s in shapes.iter() {
            let lines = util::to_lines(s);
            let c = lines[0].chars().nth(1).unwrap();
            if c == '#' {
                locks.push(parse_lock(&lines));
            } else {
                keys.push(parse_key(&lines));
            }
        }

        Problem {
            keys,
            locks,
        }
    }

    fn solve(&self) -> usize {
        let mut result = 0;

        for key in self.keys.iter() {
            for lock in self.locks.iter() {
                if key_matches(&key, &lock) {
                    result += 1;
                }
            }
        }

        result
    }
}

fn parse_lock(l: &[String]) -> Vec<usize> {
    let mut heights = vec![0, 0, 0, 0, 0];
    for x in 0..l[0].len() {
        for y in 0..l.len() {
            let c = l[y].chars().nth(x).unwrap();
            if c != '#' {
                heights[x] = y - 1;
                break;
            }
        }
    }
    heights
}

fn parse_key(l: &[String]) -> Vec<usize> {
    let mut heights = vec![0, 0, 0, 0, 0];
    for x in 0..l[0].len() {
        for y in (0..l.len()).rev() {
            let c = l[y].chars().nth(x).unwrap();
            if c != '#' {
                heights[x] = 5 - y;
                break;
            }
        }
    }
    heights
}

fn key_matches(key: &[usize], lock: &[usize]) -> bool {
    for i in 0..key.len() {
        if key[i] + lock[i] > 5 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    ";

    #[test]
    fn test_part1_ex1() {
        assert_eq!(part1(INPUT), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "0");
    }
}
