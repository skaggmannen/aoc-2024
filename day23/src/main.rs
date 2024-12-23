use std::collections::{HashMap, HashSet};

fn main() {
    let input = util::read_input("day23/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let problem = Problem::from(input);

    format!("{}", problem.count_triads())
}

fn part2(input: &str) -> String {
    let problem = Problem::from(input);

    format!("{}", problem.largest_network().join(","))
}

struct Problem {
    computers: HashMap<String, Vec<String>>,
}

impl Problem {
    fn from(data: &str) -> Self {
        let lines = util::to_lines(data);
        
        let pairs: Vec<_> = lines
        .iter()
        .map(|l| l.split_once("-").unwrap())
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();

        let mut computers = HashMap::new();
        for (a, b) in pairs.iter() {
            computers.entry(a.clone()).or_insert(Vec::new()).push(b.clone());
            computers.entry(b.clone()).or_insert(Vec::new()).push(a.clone());
        }

        Problem {
            computers,
        }
    }

    fn count_triads(&self) -> usize {
        let mut triads = HashSet::new();
        for (a, others) in self.computers.iter() {
            for i in 0..others.len() {
                let b = &others[i];
                for j in i+1..others.len() {
                    let c = &others[j];
                    if self.computers[b].contains(c) {
                        let mut triad = vec![a.clone(), b.clone(), c.clone()];
                        triad.sort();
                        triads.insert(triad);
                    }
                }
            }
        }

        let mut result = 0;
        for t in triads.iter() {
            if t.iter().filter(|v|v.starts_with("t")).count() > 0 {
                result += 1;
            }
        }
        

        result
    }
    
    fn largest_network(&self) -> Vec<String> {
        let mut networks = HashSet::new();
        for (a, others) in self.computers.iter() {
            for i in 0..others.len() {
                let b = &others[i];
                let mut network = HashSet::new();
                network.insert(a.clone());
                network.insert(b.clone());

                'outer: for j in i+1..others.len() {
                    let c = &others[j];
                    for n in network.iter() {
                        if !self.computers[c].contains(&n) {
                            continue 'outer;
                        }
                    }
                    network.insert(c.clone());
                }

                let mut network: Vec<String> = network.into_iter().collect();
                network.sort();
                networks.insert(network);
            }
        }

        networks.iter().max_by(|a,b|a.len().cmp(&b.len())).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    ";

    #[test]
    fn test_part1_ex1() {
        assert_eq!(part1(INPUT), "7");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "co,de,ka,ta");
    }
}
