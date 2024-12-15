use std::{collections::HashSet, fmt::Debug};

fn main() {
    let input = util::read_input("day15/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut map = Map::parse(input);

    format!("{}", map.solve())
}

fn part2(input: &str) -> String {
    let mut map = WideMap::parse(input);

    format!("{}", map.solve())
}

struct Map {
    size: Point,
    boxes: HashSet<Point>,
    walls: HashSet<Point>,
    robot: Point,
    instructions: Vec<Point>,
}

impl Map {
    fn solve(&mut self) -> i32 {
        let instructions = self.instructions.clone();
        for d in instructions.iter() {
            self.move_robot(*d);
        }

        let mut result = 0;
        for (x, y) in self.boxes.iter() {
            result += x + y*100;
        }
        result
    }

    fn move_robot(&mut self, (dx, dy): Point) {
        let (x, y) = self.robot;
        let next = (x+dx, y+dy);

        if let Some(&b) = self.boxes.get(&next) {
            if self.push_box(b, (dx, dy)) {
                self.robot = next;
            }
        } else if !self.walls.contains(&next) {
            self.robot = next;
        }
    }

    fn push_box(&mut self, (x, y): Point, (dx, dy): Point) -> bool {
        let mut to_push = Vec::new();

        if let Some(&b) = self.boxes.get(&(x, y)) {
            let mut colliding_boxes = Vec::new();
            colliding_boxes.push(b);

            while let Some(b) = colliding_boxes.pop() {
                let (x, y) = b;
                
                to_push.push(b);

                let next = (x+dx, y+dy);

                if self.walls.contains(&next) {
                    return false;
                } else if let Some(&b) = self.boxes.get(&next) {
                    colliding_boxes.push(b);
                }
            }
        }

        for b in to_push.iter() {
            self.boxes.remove(b);
        }
        for &(x, y) in to_push.iter() {
            self.boxes.insert((x+dx, y+dy));
        }
        
        true
    }

    fn parse(data: &str) -> Self {
        let (map, instr) = data.split_once("\n\n").unwrap();
        let cells: Vec<Vec<char>> = map
            .lines()
            .filter_map(|l| util::trim_space(l))
            .map(|s| s.chars().collect())
            .collect();

        let mut robot = None;
        let mut boxes = HashSet::new();
        let mut walls = HashSet::new();

        for y in 0..cells.len() {
            for x in 0..cells[y].len() {
                match cells[y][x] {
                    '#' => {
                        let pos = (x as i32, y as i32);
                        walls.insert(pos);
                    }
                    'O' => {
                        let pos = (x as i32, y as i32);
                        boxes.insert(pos);
                    }
                    '@' => {
                        robot = Some((x as i32, y as i32));
                    }
                    _ => {}
                }
                
            }
        }

        Map {
            size: (cells[0].len() as i32, cells.len() as i32),
            robot: robot.unwrap(),
            boxes,
            walls,
            instructions: instr
                .trim()
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| {
                    match c {
                        '^' => (0, -1),
                        '>' => (1, 0),
                        'v' => (0, 1),
                        '<' => (-1, 0),
                        _ => panic!("Invalid direction {}", c),
                    }
                })
                .collect(),
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (width, height) = self.size;
        for y in 0..height {
            for x in 0..width {
                let pos = (x, y);
                if self.walls.contains(&pos) {
                    write!(f, "#")?;
                } else if self.boxes.contains(&pos) {
                    write!(f, "O")?;
                } else if pos == self.robot {
                    write!(f, "@")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}


struct WideMap {
    size: Point,
    boxes: HashSet<Point>,
    walls: HashSet<Point>,
    robot: Point,
    instructions: Vec<Point>,
}

impl WideMap {
    fn solve(&mut self) -> i32 {
        let mut i = 0;
        let instructions = self.instructions.clone();
        for d in instructions.iter() {
            i += 1;

            self.move_robot(*d);
        }

        let mut result = 0;
        for (x, y) in self.boxes.iter() {
            result += x + y*100;
        }
        result
    }

    fn move_robot(&mut self, (dx, dy): Point) {
        let (x, y) = self.robot;
        let next = (x+dx, y+dy);

        if let Some(&b) = self.get_box(next) {
            if self.push_box(b, (dx, dy)) {
                self.robot = next;
            }
        } else if !self.get_wall(next).is_some() {
            self.robot = next;
        }
    }

    fn push_box(&mut self, (x, y): Point, (dx, dy): Point) -> bool {
        let mut to_push = Vec::new();

        if let Some(&b) = self.get_box((x, y)) {
            let mut colliding_boxes = Vec::new();
            colliding_boxes.push(b);

            while let Some(b) = colliding_boxes.pop() {
                let (x, y) = b;
                
                to_push.push(b);

                if dx > 0 {
                    let next = (x+2, y);
                    if self.get_wall(next).is_some() {
                        return false;
                    }
                    if let Some(&b) = self.get_box(next) {
                        colliding_boxes.push(b);
                    }
                } else if dx < 0 {
                    let next = (x-1, y);
                    if self.get_wall(next).is_some() {
                        return false;
                    }
                    if let Some(&b) = self.get_box(next) {
                        colliding_boxes.push(b);
                    }
                } else {
                    let first = (x, y+dy);
                    let second = (x+1, y+dy);

                    if self.get_wall(first).is_some() {
                        return false;
                    } else if self.get_wall(second).is_some() {
                        return false;
                    }

                    match (self.get_box(first), self.get_box(second)) {
                        (Some(&b1), Some(&b2)) if b1 != b2 => {
                            colliding_boxes.push(b1);
                            colliding_boxes.push(b2);
                        },
                        (Some(&b), _) | (_, Some(&b)) => {
                            colliding_boxes.push(b);
                        },
                        _ => {},
                    }
                }
            }
        }

        for b in to_push.iter() {
            self.boxes.remove(b);
        }
        for &(x, y) in to_push.iter() {
            self.boxes.insert((x+dx, y+dy));
        }
        
        true
    }

    fn get_box(&self, (x, y): Point) -> Option<&Point> {
        self.boxes.get(&(x, y))
            .or(self.boxes.get(&(x-1,y)))
    }

    fn get_wall(&self, (x, y): Point) -> Option<&Point> {
        self.walls.get(&(x, y))
            .or(self.walls.get(&(x-1, y)))
    }

    fn parse(data: &str) -> Self {
        let (map, instr) = data.split_once("\n\n").unwrap();
        let cells: Vec<Vec<char>> = map
            .lines()
            .filter_map(|l| util::trim_space(l))
            .map(|s| s.chars().collect())
            .collect();

        let mut robot = None;
        let mut boxes = HashSet::new();
        let mut walls = HashSet::new();

        for y in 0..cells.len() {
            for x in 0..cells[y].len() {
                match cells[y][x] {
                    '#' => {
                        let pos = ((x as i32)*2, y as i32);
                        walls.insert(pos);
                    }
                    'O' => {
                        let pos = ((x as i32)*2, y as i32);
                        boxes.insert(pos);
                    }
                    '@' => {
                        robot = Some(((x as i32)*2, y as i32));
                    }
                    _ => {}
                }
                
            }
        }

        WideMap {
            size: ((cells[0].len() as i32)*2, cells.len() as i32),
            robot: robot.unwrap(),
            boxes,
            walls,
            instructions: instr
                .trim()
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| {
                    match c {
                        '^' => (0, -1),
                        '>' => (1, 0),
                        'v' => (0, 1),
                        '<' => (-1, 0),
                        _ => panic!("Invalid direction {}", c),
                    }
                })
                .collect(),
        }
    }
}

impl Debug for WideMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (width, height) = self.size;
        for y in 0..height {
            for x in 0..width {
                let pos = (x, y);
                if self.get_wall(pos).is_some() {
                    write!(f, "#")?;
                } else if let Some(&(b_x, _)) = self.get_box(pos) {
                    if b_x == x {
                        write!(f, "[")?;
                    } else {
                        write!(f, "]")?;
                    }
                } else if pos == self.robot {
                    write!(f, "@")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

type Point = (i32, i32);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "10092");
    }

    #[test]
    fn test_part1_ex1() {
        const INPUT: &str = "
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<
        ";
        assert_eq!(part1(INPUT), "2028");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "9021");
    }

    #[test]
    fn test_part2_ex1() {
        const INPUT: &str = "
            #######
            #...#.#
            #.....#
            #..OO@#
            #..O..#
            #.....#
            #######

            <vv<<^^<<^^
        ";
        assert_eq!(part2(INPUT), "618");
    }

    #[test]
    fn test_part2_ex2() {
        const INPUT: &str = "
            #######
            #.....#
            #.....#
            #.@O..#
            #..#O.#
            #...O.#
            #..O..#
            #.....#
            #######

            >><vvv>v>^^^
        ";

        assert_eq!(part2(INPUT), "1430");
    }

    #[test]
    fn test_part2_ex4() {
        const INPUT: &str = "
        #######
        #.....#
        #..O..#
        #@OO..#
        #..O..#
        #..O..#
        #.....#
        #######

        >>^^>>v
        ";

        assert_eq!(part2(INPUT), "2230");
    }

    #[test]
    fn test_part2_ex5() {
        const INPUT: &str = "
        ############
        #OO.O....#.#
        #O.OO.O....#
        #..O.#O..O##
        #OOO.......#
        #...O@O#O.O#
        #......O...#
        ##....O.O..#
        #.....OO...#
        #..#.O..O.##
        #O.O.#...O.#
        ############

        ^<><>^<^v>^>vv^>><^>>v^v^vv>^v^>>^^<^^^^vv<<^>v^><<<<v>^>v>^^<^^vv^^v<
        <<^^v^<<>v>v<^<vv^vv>v^>v^<<v<vv>v>v<>^v><v<<>v<^<vvvv><>>>v>^v>^<>v^^
        ^>v>>^<^<><v>>><>>vv<<<v^v>v^v^^vv>v>v>vvv>^^<^^vv>^v<>v^^^>^v^><^vv^>
        ^<>v>>^>vv<v^v<^^vv<^><vv^<^v^^vv><<^^^^<<^v>v>>^v^^>>v^v<v<><^v^>v<<<
        >>^<^v<><^<v>>>v^<<v>^>v><<vvv<<^<v>^<vvv>v>vv^<^^>>^^vv^^v<^<^>v^<>>^
        <^^<>>v^<><<^>v<^>^<v<>>^<<>^<>vv<>v>vv>>>^<<v^^^<<^vv<<vv^^^^^v>v>><<
        vv<^vv^^^<vv<^vv<^vv<<<><v>><vv^<>^v^>><v^>><<^^>vv><>v<<<v>v^vv>>>^^>
        ^<^v^v<<^><vv^vv>v^>v^<<v<vv>>^<>vv<>v>vv>>>^<<v^^v<>^v><v<<>v<^<vvvv>
        ";

        assert_eq!(part2(INPUT), "13570");
    }

    #[test]
    fn test_part2_ex6() {
        const INPUT: &str = "
        ######
        #.####
        #.OO@#
        #.O..#
        #....#
        ######

        <<vv<<^
        ";

        assert_eq!(part2(INPUT), "");
    }

    #[test]
    fn test_part2_ex7() {
        const INPUT: &str = "
        ########
        ##..####
        ##[]..##
        ##.[].##
        ##..[]##
        ##..@.##
        ########

        ^
        ";

        assert_eq!(part2(INPUT), "");
    }

    #[test]
    fn test_part2_ex8() {
        const INPUT: &str = "

        ########
        ##....##
        ##.[].##
        ####[]##
        ##..@.##
        ########

        ^
        ";

        assert_eq!(part2(INPUT), "");
    }
}
