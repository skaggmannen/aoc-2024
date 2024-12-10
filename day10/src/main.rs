use std::collections::HashSet;

fn main() {
    let input = util::read_input("day10/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(data: &str) -> String {
    let map = parse(data);
    let trail_heads = find_trail_heads(&map);

    let result: u32 = trail_heads.iter().map(|head| {
        let mut visited = HashSet::new();
        trail_score(&map, *head, &mut visited)
    }).sum();

    format!("{}", result)
}

fn part2(data: &str) -> String {
    let map = parse(data);
    let trail_heads = find_trail_heads(&map);

    let result: u32 = trail_heads.iter().map(|head| {
        trail_rating(&map, *head)
    }).sum();

    format!("{}", result)
}

fn parse(data: &str) -> Vec<Vec<u32>> {
    data.trim()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap_or(10)).collect())
        .collect()
}

fn find_trail_heads(map: &Vec<Vec<u32>>) -> Vec<(i32, i32)> {
    let mut heads = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 0 {
                heads.push((x as i32, y as i32));
            }
        }
    }
    heads
}

fn trail_score(map: &Vec<Vec<u32>>, pos: (i32, i32), visited: &mut HashSet<(i32, i32)>) -> u32 {
    if visited.contains(&pos) {
        return 0;
    }
    visited.insert(pos);

    let (x, y) = pos;
    let curr = map[y as usize][x as usize];
    if curr == 9 {
        return 1;
    }

    let directions = [
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
    ];

    directions.iter().map(|d| {
        let (dx, dy) = d;
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || ny < 0 || nx >= map[0].len() as i32 || ny >= map.len() as i32 {
            return 0;
        }

        let next = map[ny as usize][nx as usize];

        if next == curr + 1 {
            return trail_score(map, (nx, ny), visited);
        } else {
            return 0;
        }
    }).sum()
}

fn trail_rating(map: &Vec<Vec<u32>>, pos: (i32, i32)) -> u32 {
    let (x, y) = pos;
    let curr = map[y as usize][x as usize];
    if curr == 9 {
        return 1;
    }

    let directions = [
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
    ];

    directions.iter().map(|d| {
        let (dx, dy) = d;
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || ny < 0 || nx >= map[0].len() as i32 || ny >= map.len() as i32 {
            return 0;
        }

        let next = map[ny as usize][nx as usize];

        if next == curr + 1 {
            return trail_rating(map, (nx, ny));
        } else {
            return 0;
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "36");
    }

    #[test]
    fn test_part1_ex1() {
        let input = "
            ...0...
            ...1...
            ...2...
            6543456
            7.....7
            8.....8
            9.....9
        ";
        assert_eq!(part1(input), "2");
    }

    #[test]
    fn test_part1_ex2() {
        let input = "
            ..90..9
            ...1.98
            ...2..7
            6543456
            765.987
            876....
            987....
        ";
        assert_eq!(part1(input), "4");
    }

    #[test]
    fn test_part1_ex3() {
        let input = "
            10..9..
            2...8..
            3...7..
            4567654
            ...8..3
            ...9..2
            .....01
        ";
        assert_eq!(part1(input), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "81");
    }
}
