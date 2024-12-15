
fn main() {
    let input = util::read_input("day13/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(data: &str) -> String {
    let machines = parse(data);

    let result = machines.iter()
        .filter_map(|m| m.win())
        .sum::<i64>();

    format!("{}", result)
}

fn part2(data: &str) -> String {
    let mut machines = parse(data);

    let result = machines.iter_mut()
        .filter_map(|m| {
            m.prize.0 += 10_000_000_000_000;
            m.prize.1 += 10_000_000_000_000;
            m.win()
        })
        .sum::<i64>();

    format!("{}", result)
}

fn parse(data: &str) -> Vec<Machine> {
    data.split("\n\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| parse_machine(s))
        .collect()
}

fn parse_machine(data: &str) -> Machine {
    let mut lines = data.lines().map(|s| s.trim()).filter(|s| !s.is_empty());
    let a = lines
        .next()
        .unwrap()
        .split_once(", ")
        .map(|(s1, s2)| {
            let dx = s1.strip_prefix("Button A: X+").unwrap().parse().unwrap();
            let dy = s2.strip_prefix("Y+").unwrap().parse().unwrap();
            Button {dx, dy, cost: 3}
        })
        .unwrap();
    let b = lines
        .next()
        .unwrap()
        .split_once(", ")
        .map(|(s1, s2)| {
            let dx = s1.strip_prefix("Button B: X+").unwrap().parse().unwrap();
            let dy = s2.strip_prefix("Y+").unwrap().parse().unwrap();
            Button {dx, dy, cost: 1}
        })
        .unwrap();
    let prize = lines
        .next()
        .unwrap()
        .split_once(", ")
        .map(|(s1, s2)| {
            let x = s1.strip_prefix("Prize: X=").unwrap().parse().unwrap();
            let y = s2.strip_prefix("Y=").unwrap().parse().unwrap();
            (x, y)
        })
        .unwrap();

    Machine { a, b, prize }
}

#[derive(Debug)]
struct Machine {
    a: Button,
    b: Button,
    prize: (i64, i64),
}

#[derive(Debug)]
struct Button {
    dx: i64,
    dy: i64,
    cost: i64,
}

impl Machine {
    fn win(&self) -> Option<i64> {
        let (a_x, a_y) = (self.a.dx, self.a.dy);
        let (b_x, b_y) = (self.b.dx, self.b.dy);
        let (p_x, p_y) = self.prize;

        let d1 = a_x*p_y - a_y*p_x;
        let d2 = a_x*b_y - a_y*b_x;

        // Check if there's an even number of B presses that results in the prize.
        if d1 % d2 != 0 {
            return None;
        }

        let j = (a_x*p_y - a_y*p_x)/(a_x*b_y - a_y*b_x);
        let d3 = p_x - b_x*j;

        // Check if there's an even number of A presses that results in the prize.
        if d3 % a_x != 0 {
            return None;
        }

        let i = d3/a_x;

        Some(i*self.a.cost + j*self.b.cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "480");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "80");
    }
}
