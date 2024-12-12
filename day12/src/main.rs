use std::collections::HashSet;

fn main() {
    let input = util::read_input("day12/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(data: &str) -> String {
    let map = parse(data);

    let cost = fence_cost(&map);

    format!("{}", cost)
}

fn part2(data: &str) -> String {
    let map = parse(data);

    let cost = fence_cost_part2(&map);

    format!("{}", cost)
}

fn parse(data: &str) -> Vec<Vec<char>> {
    data.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

fn fence_cost(map: &Vec<Vec<char>>) -> u64 {
    let mut visited = HashSet::new();

    let mut result = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if visited.contains(&(row, col)) {
                continue;
            }

            result += find_region(&map, (row, col), &mut visited);
        }
    }
    result
}

fn find_region(map: &Vec<Vec<char>>, start: Point, visited: &mut HashSet<Point>) -> u64 {
    let mut area = 0;
    let mut perimeter = 0;
    let mut queue = vec![start];
    let (row, col) = start;
    let crop_type = map[row][col];

    while let Some((row, col)) = queue.pop() {
        if visited.contains(&(row, col)) {
            continue;
        }

        if map[row][col] != crop_type {
            perimeter += 1;
            continue;
        }

        visited.insert((row, col));

        area += 1;

        if row == 0 || map[row - 1][col] != crop_type {
            // Count the top edge
            perimeter += 1;
        } else if !visited.contains(&(row - 1, col)) {
            // Visit the cell above
            queue.push((row - 1, col));
        }

        if row == map.len() - 1 || map[row + 1][col] != crop_type {
            // Count the bottom edge
            perimeter += 1;
        } else if !visited.contains(&(row + 1, col)) {
            // Visit the cell below
            queue.push((row + 1, col));
        }

        if col == 0 || map[row][col - 1] != crop_type {
            // Count the left edge
            perimeter += 1;
        } else if !visited.contains(&(row, col - 1)) {
            // Visit the cell to the left
            queue.push((row, col - 1));
        }

        if col == map[row].len() - 1 || map[row][col + 1] != crop_type {
            // Count the right edge
            perimeter += 1;
        } else if !visited.contains(&(row, col + 1)) {
            // Visit the cell to the right
            queue.push((row, col + 1));
        }
    }

    println!(
        "Region {{ crop_type: {:?}, area: {}, perimeter: {} }}",
        crop_type, area, perimeter
    );

    area as u64 * perimeter as u64
}

fn fence_cost_part2(map: &Vec<Vec<char>>) -> u64 {
    let mut visited = HashSet::new();

    let mut result = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if visited.contains(&(row, col)) {
                continue;
            }

            result += find_region_part2(&map, (row, col), &mut visited);
        }
    }
    result
}

fn find_region_part2(map: &Vec<Vec<char>>, start: Point, visited: &mut HashSet<Point>) -> u64 {
    let mut area = 0;
    let mut sides = 0;
    let mut queue = vec![start];
    let (row, col) = start;
    let crop_type = map[row][col];

    while let Some((row, col)) = queue.pop() {
        if visited.contains(&(row, col)) {
            continue;
        }

        visited.insert((row, col));

        area += 1;

        let pos = MapPos {
            map,
            row,
            col,
            crop_type,
        };

        if pos.north_west_corner() {
            sides += 1;
        }
        if pos.north_east_corner() {
            sides += 1;
        }
        if pos.south_east_corner() {
            sides += 1;
        }
        if pos.south_west_corner() {
            sides += 1;
        }

        if let Some(point) = pos.north() {
            if !visited.contains(&point) {
                // Visit the cell to the north.
                queue.push(point);
            }
        }

        if let Some(point) = pos.east() {
            if !visited.contains(&point) {
                // Visit the cell to the east.
                queue.push(point);
            }
        }

        if let Some(point) = pos.south() {
            if !visited.contains(&point) {
                // Visit the cell to the south.
                queue.push(point);
            }
        }

        if let Some(point) = pos.west() {
            if !visited.contains(&point) {
                // Visit the cell to the west.
                queue.push(point);
            }
        }
    }

    println!(
        "Region {{ crop_type: {:?}, area: {}, sides: {} }}",
        crop_type, area, sides
    );

    area as u64 * sides as u64
}

struct MapPos<'a> {
    map: &'a Vec<Vec<char>>,
    row: usize,
    col: usize,
    crop_type: char,
}

impl<'a> MapPos<'a> {
    fn north(&self) -> Option<Point> {
        if self.row == 0 || self.map[self.row - 1][self.col] != self.crop_type {
            None
        } else {
            Some((self.row - 1, self.col))
        }
    }
    fn north_east(&self) -> Option<Point> {
        if self.row == 0
            || self.col == self.map[self.row].len() - 1
            || self.map[self.row - 1][self.col + 1] != self.crop_type
        {
            None
        } else {
            Some((self.row - 1, self.col + 1))
        }
    }

    fn east(&self) -> Option<Point> {
        if self.col == self.map[self.row].len() - 1
            || self.map[self.row][self.col + 1] != self.crop_type
        {
            None
        } else {
            Some((self.row, self.col + 1))
        }
    }

    fn south_east(&self) -> Option<Point> {
        if self.row == self.map.len() - 1
            || self.col == self.map[self.row].len() - 1
            || self.map[self.row + 1][self.col + 1] != self.crop_type
        {
            None
        } else {
            Some((self.row + 1, self.col + 1))
        }
    }

    fn south(&self) -> Option<Point> {
        if self.row == self.map.len() - 1 || self.map[self.row + 1][self.col] != self.crop_type {
            None
        } else {
            Some((self.row + 1, self.col))
        }
    }

    fn south_west(&self) -> Option<Point> {
        if self.row == self.map.len() - 1
            || self.col == 0
            || self.map[self.row + 1][self.col - 1] != self.crop_type
        {
            None
        } else {
            Some((self.row + 1, self.col - 1))
        }
    }

    fn west(&self) -> Option<Point> {
        if self.col == 0 || self.map[self.row][self.col - 1] != self.crop_type {
            None
        } else {
            Some((self.row, self.col - 1))
        }
    }

    fn north_west(&self) -> Option<Point> {
        if self.row == 0 || self.col == 0 || self.map[self.row - 1][self.col - 1] != self.crop_type
        {
            None
        } else {
            Some((self.row - 1, self.col - 1))
        }
    }

    fn north_west_corner(&self) -> bool {
        (
            // Normal corner
            self.north().is_none() && self.west().is_none()
        ) || (
            // Inverted corner
            self.north_west().is_none() && self.north().is_some() && self.west().is_some()
        )
    }

    fn north_east_corner(&self) -> bool {
        (
            // Normal corner
            self.north().is_none() && self.east().is_none()
        ) || (
            // Inverted corner
            self.north_east().is_none() && self.north().is_some() && self.east().is_some()
        )
    }

    fn south_east_corner(&self) -> bool {
        (
            // Normal corner
            self.south().is_none() && self.east().is_none()
        ) || (
            // Inverted corner
            self.south_east().is_none() && self.south().is_some() && self.east().is_some()
        )
    }

    fn south_west_corner(&self) -> bool {
        (
            // Normal corner
            self.south().is_none() && self.west().is_none()
        ) || (
            // Inverted corner
            self.south_west().is_none() && self.south().is_some() && self.west().is_some()
        )
    }
}

type Point = (usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        const INPUT: &str = "
            AAAA
            BBCD
            BBCC
            EEEC
        ";

        assert_eq!(part1(INPUT), "140");
    }

    #[test]
    fn test_part1_ex2() {
        const INPUT: &str = "
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO
        ";

        assert_eq!(part1(INPUT), format!("{}", 21 * 36 + 4 * 4));
    }

    #[test]
    fn test_part1_ex3() {
        const INPUT: &str = "
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        ";

        assert_eq!(part1(INPUT), format!("{}", 1930));
    }

    #[test]
    fn test_part2_ex1() {
        const INPUT: &str = "
            AAAA
            BBCD
            BBCC
            EEEC
        ";

        assert_eq!(part2(INPUT), "80");
    }

    #[test]
    fn test_part2_ex2() {
        const INPUT: &str = "
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO
        ";

        assert_eq!(part2(INPUT), "436");
    }

    #[test]
    fn test_part2_ex3() {
        const INPUT: &str = "
            EEEEE
            EXXXX
            EEEEE
            EXXXX
            EEEEE
        ";

        assert_eq!(part2(INPUT), "236");
    }

    #[test]
    fn test_part2_ex4() {
        const INPUT: &str = "
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA
        ";

        assert_eq!(part2(INPUT), "368");
    }
}
