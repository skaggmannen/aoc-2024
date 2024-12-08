fn main() {
    let input = util::read_input("day4/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

pub fn part1(data: &str) -> String {
    let grid = parse_grid(data);

    let mut result = 0;
    for row in 0..grid.len() {
        let l = &grid[row];
        for col in 0..l.len() {
            for (dx, dy) in [
                (0, 1),
                (1, 0),
                (1, 1),
                (-1, 0),
                (0, -1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ] {
                if find_xmas(&grid, 'X', row as i32, col as i32, dx, dy) {
                    result += 1;
                }
            }
        }
    }

    format!("{}", result)
}

fn find_xmas(grid: &Vec<Vec<char>>, needle: char, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    if x < 0 || y < 0 || y >= grid.len() as i32 || x >= grid[0].len() as i32 {
        return false;
    }

    match grid[x as usize][y as usize] {
        'X' => needle == 'X' && find_xmas(grid, 'M', x + dx, y + dy, dx, dy),
        'M' => needle == 'M' && find_xmas(grid, 'A', x + dx, y + dy, dx, dy),
        'A' => needle == 'A' && find_xmas(grid, 'S', x + dx, y + dy, dx, dy),
        'S' => needle == 'S',
        _ => false,
    }
}

pub fn part2(data: &str) -> String {
    let grid = parse_grid(data);

    let mut result = 0;

    for row in 1..(grid.len()-1) {
        let l = &grid[row];
        for col in 1..(l.len()-1) {
            if find_x_mas(&grid, row, col) {
                result += 1;
            }
        }
    }

    format!("{}", result)
}

fn find_x_mas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if grid[row][col] != 'A' {
        return false;
    }
    
    let down_left = match (grid[row-1][col-1], grid[row+1][col+1]) {
        ('M', 'S') => true,
        ('S', 'M') => true,
        _ => false,
    };

    let down_right = match (grid[row-1][col+1], grid[row+1][col-1]) {
        ('M', 'S') => true,
        ('S', 'M') => true,
        _ => false,
    };

    down_left && down_right
}

fn parse_grid(data: &str) -> Vec<Vec<char>> {
    data.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        ";
        assert_eq!(part1(INPUT), "18");
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        ";

        assert_eq!(part2(INPUT), "9");
    }
}
