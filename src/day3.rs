use regex::Regex;

pub fn part1(data: &str) -> String {
    const PATTERN: &str = r"mul\((\d+),(\d+)\)";

    let re = Regex::new(PATTERN).unwrap();

    let result = re.captures_iter(data).map(|m| {
        let a = m.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let b = m.get(2).unwrap().as_str().parse::<i32>().unwrap();
        a * b
    }).sum::<i32>();

    format!("{}", result)
}

pub fn part2(data: &str) -> String {
    const PATTERN: &str = r"(do)\(\)|(don't)\(\)|(mul)\((\d+),(\d+)\)";

    let re = Regex::new(PATTERN).unwrap();
    let mut enabled = true;

    let result = re.captures_iter(data).map(|m| {
        let mut groups = m.iter().skip(1).filter(|x| x.is_some()).map(|x| x.unwrap());
        let op = groups.next().unwrap().as_str();
        match op {
            "do" => { 
                enabled = true; 
                return 0; 
            },
            "don't" => { 
                enabled = false; 
                return 0; 
            },
            "mul" => {
                if enabled {
                    let a = groups.next().unwrap().as_str().parse::<i32>().unwrap();
                    let b = groups.next().unwrap().as_str().parse::<i32>().unwrap();
                    return a * b;
                } else {
                    return 0;
                }
            },
            _ => return 0,
        }
    }).sum::<i32>();

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "
            xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        ";
        assert_eq!(part1(INPUT), "161");
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "
            xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        ";

        assert_eq!(part2(INPUT), "48");
    }
}
