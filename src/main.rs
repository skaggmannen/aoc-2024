use std::env::args;

fn main() {
    let path = args().nth(1).unwrap();
    let input = util::read_input(&path);

    match path.as_str() {
        "src/day1.txt" => {
            println!("Part 1: {}", day1::part1(&input));
            println!("Part 2: {}", day1::part2(&input));
        }
        _ => println!("Unsupported day!"),
    }
}

mod day1;
mod util;
