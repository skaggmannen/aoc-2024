fn main() {
    let input = util::read_input("day7/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(data: &str) -> String {
    let lines = parse_input(data);
    
    const OPERATORS: [&'static str; 2] = ["+", "*"];

    let mut result = 0;
    for (answer, operands) in lines {
        let head = *operands.first().unwrap();
        let tail = &operands[1..];
        for op in OPERATORS {
            if calculate(head, op, tail, answer, &OPERATORS) {
                result += answer;
                break;
            }
        }
    }

    format!("{}", result)
}

fn part2(data: &str) -> String {
    
    let lines = parse_input(data);
    
    const OPERATORS: [&'static str; 3] = ["+", "*", "||"];

    let mut result = 0;
    for (answer, operands) in lines {
        let head = *operands.first().unwrap();
        let tail = &operands[1..];
        for op in OPERATORS {
            if calculate(head, op, tail, answer, &OPERATORS) {
                result += answer;
                break;
            }
        }
    }

    format!("{}", result)
}

fn parse_input(data: &str) -> Vec<(i64, Vec<i64>)> {
    data.trim().lines().map(|line| {
        let (first, second) = line.trim().split_once(": ").unwrap();
        let answer = first.parse::<i64>().unwrap();
        let operands = second.split_whitespace().map(|op| op.parse::<i64>().unwrap()).collect::<Vec<_>>();
        (answer, operands)
    }).collect::<Vec<_>>()
}

fn calculate(head: i64, operator: &'static str, tail: &[i64], answer: i64, operators: &[&'static str]) -> bool {
    let next = match operator {
        "+" => head + tail[0],
        "*" => head * tail[0],
        "||" => format!("{}{}", head, tail[0]).parse::<i64>().unwrap(),
        _ => panic!("Invalid operator"),
    };
    let tail = &tail[1..];
    if tail.is_empty() {
        return next == answer;
    }

    for op in operators {
        if calculate(next, op, tail, answer, operators) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "3749");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "11387");
    }
}
