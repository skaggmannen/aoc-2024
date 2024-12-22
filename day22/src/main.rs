use std::collections::HashMap;


fn main() {
    let input = util::read_input("day22/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let buyers: Vec<_> = util::to_lines(input)
        .iter()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    let mut result = 0;
    for v in buyers.iter() {
        let mut rnd = Random { secret: *v };
        for _ in 0..2000 {
            rnd.next();
        }
        result += rnd.secret;
    }

    format!("{}", result)
}

fn part2(input: &str) -> String {
    let buyers: Vec<_> = util::to_lines(input)
        .iter()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    let secrets: Vec<_> = buyers.iter().map(|b| {
        Random{ secret: *b }.generate(2000)
    }).collect();

    let prices: Vec<_> = secrets.iter().map(|s| {
        prices(&s)
    }).collect();

    let changes: Vec<_> = prices.iter().map(|p| {
        changes(&p)
    }).collect();

    let sequences: Vec<_> = buyers.iter().enumerate().map(|(i, _)| {
        build_sequences(&prices[i], &changes[i])
    }).collect();


    let mut total_bananas = Vec::new();
    for i in 0..buyers.len() {
        for (w1, &v1) in sequences[i].iter() {
            let mut total = v1;

            for j in 1..buyers.len() {
                if j == i {
                    continue;
                }

                if let Some(v2) = sequences[j].get(w1) {
                    total += v2;
                }
            }

            total_bananas.push(total);
        }
    }
    

    format!("{}", total_bananas.iter().max().unwrap())
}

fn prices(secrets: &[usize]) -> Vec<usize> {
    secrets.iter().map(|s| s % 10).collect()
}

fn changes(prices: &[usize]) -> Vec<i32> {
    prices
        .windows(2)
        .map(|w| (*w.last().unwrap() as i32) - (*w.first().unwrap() as i32))
        .collect()
}

fn build_sequences(prices: &[usize], changes: &[i32]) -> HashMap<Vec<i32>, usize> {
    let mut result = HashMap::new();
    for (i, w) in changes.windows(4).enumerate() {
        if !result.contains_key(w) {
            result.insert(w.to_vec(), prices[i+4]);
        }
    }
    result
}

struct Random {
    secret: usize,
}

impl Random {
    fn generate(&mut self, count: usize) -> Vec<usize> {
        let mut result = Vec::new();
        result.push(self.secret);
        for _ in 0..count {
            self.next();
            result.push(self.secret);
        }
        result
    }

    fn next(&mut self) -> usize {
        let a = self.secret * 64;
        self.mix(a);
        self.prune();

        let b = self.secret / 32;
        self.mix(b);
        self.prune();

        let c = self.secret * 2048;
        self.mix(c);
        self.prune();

        self.secret
    }

    fn mix(&mut self, v: usize) {
        let r = self.secret ^ v;
        self.secret = r;
    }

    fn prune(&mut self) {
        self.secret = self.secret % 16777216;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        1
        10
        100
        2024
    ";

    #[test]
    fn test_random() {
        let mut random = Random { secret: 123 };
        assert_eq!(random.next(), 15887950);
        assert_eq!(random.next(), 16495136);
        assert_eq!(random.next(), 527345);
        assert_eq!(random.next(), 704524);
        assert_eq!(random.next(), 1553684);
        assert_eq!(random.next(), 12683156);
        assert_eq!(random.next(), 11100544);
        assert_eq!(random.next(), 12249484);
        assert_eq!(random.next(), 7753432);
        assert_eq!(random.next(), 5908254);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "37327623");
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "
            1
            2
            3
            2024
        ";
        assert_eq!(part2(INPUT), "23");
    }

    #[test]
    fn test_prices() {
        let mut rnd = Random { secret: 123 };
        let secrets = rnd.generate(10);
        let prices = prices(&secrets);
        let changes = changes(&prices);

        for i in 0..secrets.len() {
            print!("{:10} {}", secrets[i], prices[i]);
            if i > 0 {
                print!(" ({})", changes[i-1]);
            }
            println!();
        }
    }
}
