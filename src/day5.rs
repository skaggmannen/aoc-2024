use std::collections::HashMap;

pub fn part1(data: &str) -> String {
    let (rules, updates) = parse_input(data);

    let mut result = 0;

    for update in updates {
        if is_valid(&rules, &update) {
            result += update[update.len() / 2];
        }
    }

    format!("{}", result)
}

pub fn part2(data: &str) -> String {
    let (rules, updates) = parse_input(data);

    let mut invalid_updates = Vec::new();

    for update in updates {
        if !is_valid(&rules, &update) {
            invalid_updates.push(update);
        }
    }

    let mut result = 0;
    for mut update in invalid_updates {
        update.sort_by(|a, b| {
            if let Some(rules) = rules.get(a) {
                if rules.contains(b) {
                    return std::cmp::Ordering::Less;
                }
            }

            std::cmp::Ordering::Equal
        });

        result += update[update.len() / 2];
    }

    format!("{}", result)
}

fn parse_input(data: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules = HashMap::new();
    let mut updates = Vec::new();

    let mut parts = data.trim().split("\n\n");

    // Parse the rules block
    for line in parts.next().unwrap().lines() {
        let mut parts = line.trim().split("|");
        let key = parts.next().unwrap().parse().unwrap();
        let value = parts.next().unwrap().parse().unwrap();
        rules.entry(key).or_insert(Vec::new()).push(value);
    }

    // Parse the updates block
    for line in parts.next().unwrap().lines() {
        let line = line.trim().split(",").map(|s| s.parse().unwrap()).collect();

        updates.push(line);
    }

    (rules, updates)
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
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "143");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "123");
    }
}
