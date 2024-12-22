use std::{
    collections::HashMap,
    iter::{once, repeat_n},
};

fn main() {
    let input = util::read_input("day21/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let problem = Problem::from(input);

    format!("{}", problem.solve(3))
}

fn part2(input: &str) -> String {
    let problem = Problem::from(input);

    format!("{}", problem.solve(26))
}

struct Problem {
    codes: Vec<String>,
}

impl Problem {
    fn from(data: &str) -> Self {
        Problem {
            codes: util::to_lines(data),
        }
    }

    fn solve(&self, depth: u8) -> usize {
        let mut result = 0;
        let mut mem = Mem::new();

        for code in self.codes.iter() {
            let len = dfs(&code, depth, &type_on_num_pad, &mut mem);
            let v = (&code[..code.len() - 1]).parse::<usize>().unwrap();
            result += len * v;
        }

        result
    }
}

type Mem = HashMap<(String, u8), usize>;

fn dfs<F: Fn(char, char) -> Vec<String>>(
    s: &str,
    depth: u8,
    type_on_pad: F,
    mem: &mut Mem,
) -> usize {
    if depth == 0 {
        return s.len();
    }

    if let Some(v) = mem.get(&(s.to_string(), depth)) {
        return *v;
    }

    let mut res = 0;
    let mut from = 'A';
    for to in s.chars() {
        res += type_on_pad(from, to)
            .into_iter()
            .map(|s| dfs(&s, depth - 1, &type_on_dir_pad, mem))
            .min()
            .unwrap_or(0);
        from = to;
    }

    mem.insert((s.to_string(), depth), res);

    res
}

// type_on_dir_pad generates the directional input required to move from 
// one char to the next on the num pad.
fn type_on_num_pad(from: char, to: char) -> Vec<String> {
    let (x1, y1) = map_num_pad(from);
    let (x2, y2) = map_num_pad(to);
    let (dx, dy) = (x2 - x1, y2 - y1);

    let v_keys = if dy > 0 {
        repeat_n('v', dy as usize)
    } else {
        repeat_n('^', -dy as usize)
    };
    let h_keys = if dx > 0 {
        repeat_n('>', dx as usize)
    } else {
        repeat_n('<', -dx as usize)
    };

    if dx == 0 {
        vec![v_keys.chain(once('A')).collect()]
    } else if dy == 0 {
        vec![h_keys.chain(once('A')).collect()]
    } else if x1 == 0 && y2 == 3 {
        vec![h_keys.chain(v_keys).chain(once('A')).collect()]
    } else if y1 == 3 && x2 == 0 {
        vec![v_keys.chain(h_keys).chain(once('A')).collect()]
    } else {
        vec![
            v_keys
                .clone()
                .chain(h_keys.clone())
                .chain(once('A'))
                .collect(),
            h_keys
                .clone()
                .chain(v_keys.clone())
                .chain(once('A'))
                .collect(),
        ]
    }
}

fn type_on_dir_pad(from: char, to: char) -> Vec<String> {
    let (x1, y1) = map_dir_pad(from);
    let (x2, y2) = map_dir_pad(to);
    let (dx, dy) = (x2 - x1, y2 - y1);

    let v_keys = if dy > 0 {
        repeat_n('v', dy as usize)
    } else {
        repeat_n('^', -dy as usize)
    };
    let h_keys = if dx > 0 {
        repeat_n('>', dx as usize)
    } else {
        repeat_n('<', -dx as usize)
    };

    if dx == 0 {
        vec![v_keys.chain(once('A')).collect()]
    } else if dy == 0 {
        vec![h_keys.chain(once('A')).collect()]
    } else if x1 == 0 && y2 == 0 {
        vec![h_keys.chain(v_keys).chain(once('A')).collect()]
    } else if y1 == 0 && x2 == 0 {
        vec![v_keys.chain(h_keys).chain(once('A')).collect()]
    } else {
        vec![
            v_keys
                .clone()
                .chain(h_keys.clone())
                .chain(once('A'))
                .collect(),
            h_keys
                .clone()
                .chain(v_keys.clone())
                .chain(once('A'))
                .collect(),
        ]
    }
}

fn map_num_pad(c: char) -> (i32, i32) {
    match c {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '#' => (0, 3),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!("invalid char {}", c),
    }
}

fn map_dir_pad(c: char) -> (i32, i32) {
    match c {
        '#' => (0, 0),
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!("invalid char {}", c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        029A
        980A
        179A
        456A
        379A
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "126384");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "154115708116294");
    }
}