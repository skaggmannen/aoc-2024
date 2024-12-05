use std::env::args;

fn main() {
    let path = args().nth(1).unwrap();
    let input = util::read_input(&path);

    match path.as_str() {
        "src/day1.txt" => {
            println!("Part 1: {}", day1::part1(&input));
            println!("Part 2: {}", day1::part2(&input));
        }
        "src/day2.txt" => {
            println!("Part 1: {}", day2::part1(&input));
            println!("Part 2: {}", day2::part2(&input));
        }
        "src/day3.txt" => {
            println!("Part 1: {}", day3::part1(&input));
            println!("Part 2: {}", day3::part2(&input));
        }
        "src/day4.txt" => {
            println!("Part 1: {}", day4::part1(&input));
            println!("Part 2: {}", day4::part2(&input));
        }
        "src/day5.txt" => {
            println!("Part 1: {}", day5::part1(&input));
            println!("Part 2: {}", day5::part2(&input));
        }
        _ => println!("Unsupported day!"),
    }
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod util;
