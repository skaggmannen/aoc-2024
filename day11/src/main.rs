use std::collections::HashMap;


fn main() {
    let input = util::read_input("day11/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(data: &str) -> String {
    let stones = parse(data);

    let mutated = mutate(&stones, 25, &mut HashMap::new());

    format!("{}", mutated)
}

fn part2(data: &str) -> String {
    let stones = parse(data);

    let mutated = mutate(&stones, 75, &mut HashMap::new());

    format!("{}", mutated)
}

fn parse(data: &str) -> Vec<u64> {
    data.trim()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn mutate(stones: &[u64], generations: usize, memory: &mut HashMap<(u64, usize), usize>) -> usize {
    stones.iter().map(|s| mutate_stone(*s, generations, memory)).sum()
}

fn mutate_stone(stone: u64, generations: usize, memory: &mut HashMap<(u64, usize), usize>) -> usize {
    if let Some(mem) = memory.get(&(stone, generations)) {
        return *mem;
    }

    let result = if generations == 0 {
        1
    } else if stone == 0 {
        mutate_stone(1, generations-1, memory)
    } else if count_digits(stone) % 2 == 0 {
        let s = stone.to_string();
        let (first, second) = s.split_at(s.len()/2);
        mutate_stone(first.parse().unwrap(), generations-1, memory) + mutate_stone(second.parse().unwrap(), generations-1, memory)
    } else {
        mutate_stone(stone*2024, generations-1, memory)
    };

    memory.insert((stone, generations), result);

    result
}

fn count_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        125 17
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "55312");
    }
}
