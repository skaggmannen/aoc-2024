
fn main() {
    let input = util::read_input("dayXX/input.txt");

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

struct Problem {}

impl From<&str> for Problem {
    fn from(_data: &str) -> Self {
        Problem {}
    }
}

impl Problem {
    fn solve(&self) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        Something
    ";

    #[test]
    fn test_part1_ex1() {
        assert_eq!(part1(INPUT), "0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "0");
    }
}
