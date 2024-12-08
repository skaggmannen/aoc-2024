fn main() {
    let input = util::read_input("day2/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

pub fn part1(data: &str) -> String {
    let lines = util::to_lines(data);

    let mut safe_count = 0;
    for line in lines {
        let levels: Vec<i32> = line.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        if is_safe(&levels) {
            safe_count += 1;
        }
    }

    format!("{}", safe_count)
}

fn is_safe(levels: &[i32]) -> bool {
    let mut diff = 0;

    levels.windows(2).all(|w| {
        let first = w.get(0).unwrap();
        let second = w.get(1).unwrap();
        let d = first - second;
        
        if (d.abs() < 1) || (d.abs() > 3) || (d < 0 && diff > 0) || (d > 0 && diff < 0) {
            return false;
        }

        diff = d;

        true
    })
}

pub fn part2(data: &str) -> String {
    let lines = util::to_lines(data);

    let mut safe_count = 0;
    for line in lines {
        let levels: Vec<i32> = line.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        if is_safe_with_dampener(&levels) {
            safe_count += 1;
        }
    }

    format!("{}", safe_count)
}

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut v = Vec::new();
        v.extend_from_slice(&levels[..i]);
        v.extend_from_slice(&levels[i+1..]);
        if is_safe(&v) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "4");
    }

    #[test]
    fn test_part2_edge_case() {
        const EDGE_CASE: &str = "
            7 8 4 2 1
            3 10 2 1
        ";
        assert_eq!(part2(EDGE_CASE), "2");
    }
}
