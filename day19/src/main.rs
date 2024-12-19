use std::collections::{HashMap, HashSet};


fn main() {
    let input = util::read_input("day19/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let problem = Problem::from(input);

    format!("{}",problem.solve_part1())
}

fn part2(input: &str) -> String {
    let problem = Problem::from(input);
    
    format!("{}",problem.solve_part2())
}

struct Problem {
    patterns: HashSet<String>,
    designs: Vec<String>,
}

impl Problem {
    fn from(data: &str) -> Self {
        let (patterns, designs) = data.trim().split_once("\n\n").unwrap();

        let patterns = patterns.trim().split(", ").map(|s|s.to_string()).collect();
        let designs = util::to_lines(designs);

        Problem {
            patterns,
            designs,
        }
    }

    fn solve_part1(&self) -> usize {
        let mut result = 0;
        let mut memory = HashMap::new();
        for design in self.designs.iter() {
            if self.available_combinations(design, &mut memory) > 0 {
                result += 1;
            }
        }
        result
    }

    fn solve_part2(&self) -> usize {
        let mut result = 0;
        let mut memory = HashMap::new();
        for design in self.designs.iter() {
            result += self.available_combinations(design, &mut memory);
        }
        result
    }
    
    fn available_combinations<'a>(&self, design: &'a str, memory: &mut HashMap<String, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }
        if let Some(&result) = memory.get(design) {
            return result;
        }

        let mut result = 0;
        for pattern in self.patterns.iter() {
            if let Some(rest) = design.strip_prefix(pattern) {
                result += self.available_combinations(rest, memory);
            }
        }

        memory.insert(design.to_string(), result);

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    ";

    #[test]
    fn test_part1_ex1() {
        assert_eq!(part1(INPUT), "6");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "16");
    }
}
