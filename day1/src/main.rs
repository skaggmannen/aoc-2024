fn main() {
    let input = util::read_input("day1/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

use std::collections::HashMap;

pub fn part1(data: &str) -> String {
    let lines = util::to_lines(data);

    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in lines {
        let mut pair = line.split_ascii_whitespace();
        first_list.push(pair.next().unwrap().parse::<i32>().unwrap());
        second_list.push(pair.next().unwrap().parse::<i32>().unwrap());
    }

    first_list.sort();
    second_list.sort();

    let mut result = 0;
    for (first, second) in first_list.iter().zip(second_list) {
        result += (first - second).abs();
    }

    format!("{}", result)
}

pub fn part2(data: &str) -> String {
    let lines = util::to_lines(data);

    let mut left = Vec::new();
    let mut right_count = HashMap::<i32, i32>::new();

    for line in lines {
        let mut pair = line.split_ascii_whitespace();
        let l = pair.next().unwrap().parse::<i32>().unwrap();
        let r = pair.next().unwrap().parse::<i32>().unwrap();

        left.push(l);
        *right_count.entry(r).or_default() += 1;
    }

    let mut result = 0;
    for l in left {
        result += l * (*right_count.entry(l).or_default())
    }

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "11");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "31");
    }
}
