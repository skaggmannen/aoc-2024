
fn main() {
    let input = util::read_input("day17/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut p = Program::from(input);

    p.run();

    format!("{}", p.output.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","))
}

fn part2(input: &str) -> String {
    let p = Program::from(input);

    if let Some(a) = next_match(&p, p.raw_program.len() - 1, 0) {
        return format!("{}", a);
    }

    format!("this is impossible!")
}

fn next_match(original: &Program, start: usize, v: i64) -> Option<i64> {
    let to_match = &original.raw_program.clone()[start..];

    for i in 0..0o10 {
        let a = (v << 3) | i;

        let mut p = original.clone();
        p.a = a;
        p.run();

        if p.output == to_match {
            if start == 0 {
                return Some(a);
            } else if let Some(a) = next_match(original, start - 1, a) {
                return Some(a);
            }
        }
    }

    None
}

#[derive(Clone)]
struct Program {
    a: i64,
    b: i64,
    c: i64,

    pc: usize,
    output: Vec<i64>,
    instructions: Vec<Instruction>,
    raw_program: Vec<i64>,
}

impl Program {
    fn from(data: &str) -> Self {
        let (regs, instr) = data.trim().split_once("\n\n").unwrap();

        let mut regs = regs.lines();

        let a = regs
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Register A: ")
            .unwrap()
            .parse()
            .unwrap();
        let b = regs
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Register B: ")
            .unwrap()
            .parse()
            .unwrap();
        let c = regs
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Register C: ")
            .unwrap()
            .parse()
            .unwrap();

        let raw_program: Vec<i64> = instr
            .trim()
            .strip_prefix("Program: ")
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let instructions: Vec<Instruction> = raw_program
            .chunks(2)
            .map(|chunk| Instruction::from((chunk[0], chunk[1])))
            .collect();

        Program {
            a,
            b,
            c,
            pc: 0,
            instructions,
            output: Vec::new(),
            raw_program,
        }
    }

    fn run(&mut self) {
        while self.step() {}
    }

    fn step(&mut self) -> bool {
        if let Some(instr) = self.instructions.get(self.pc) {
            match instr {
                Instruction::ADV { combo } => {
                    let num = self.a;
                    let denom = 1 << self.combo_value(*combo);
    
                    self.a = num/denom;
                    self.pc += 1;
                },
                Instruction::BDV { combo } => {
                    let num = self.a;
                    let denom = 1 << self.combo_value(*combo);
    
                    self.b = num/denom;
                    self.pc += 1;
                },
                Instruction::CDV { combo } => {
                    let num = self.a;
                    let denom = 1 << self.combo_value(*combo);
    
                    self.c = num/denom;
                    self.pc += 1;
                },
                Instruction::BXL { literal } => {
                    self.b = self.b ^ literal;
                    self.pc += 1;
                },
                Instruction::BXC { v: _ } => {
                    self.b = self.b ^ self.c;
                    self.pc += 1;
                },
                Instruction::BST { combo } => {
                    self.b = self.combo_value(*combo) % 8;
                    self.pc += 1;
                },
                Instruction::JNZ { literal } => {
                    if self.a != 0 {
                        self.pc = *literal as usize;
                    } else {
                        self.pc += 1;
                    }
                },
                Instruction::OUT { combo } => {
                    self.output.push(self.combo_value(*combo) % 8);
                    self.pc += 1;
                },
            }

            true
        } else {
            false
        }
        
    }

    fn combo_value(&self, combo: i64) -> i64 {
        match combo {
            _ if combo <= 3 => combo,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid combo value: {}", combo),
        }
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    ADV { combo: i64 },
    BDV { combo: i64 },
    CDV { combo: i64 },
    BXL { literal: i64 },
    BXC { v: i64 },
    BST { combo: i64 },
    JNZ { literal: i64 },
    OUT { combo: i64 },
}

impl Instruction {
    fn from((op, v): (i64, i64)) -> Instruction {
        match op {
            0 => Self::ADV { combo: v },
            1 => Self::BXL { literal: v },
            2 => Self::BST { combo: v },
            3 => Self::JNZ { literal: v },
            4 => Self::BXC { v },
            5 => Self::OUT { combo: v },
            6 => Self::BDV { combo: v },
            7 => Self::CDV { combo: v },
            _ => panic!("invalid opcode: {}", op),
        }
    }
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match self {
            Instruction::ADV { combo } => format!("{},{}", 0, combo),
            Instruction::BXL { literal } => format!("{},{}", 1, literal),
            Instruction::BST { combo } => format!("{},{}", 2, combo),
            Instruction::JNZ { literal } => format!("{},{}", 3, literal),
            Instruction::BXC { v } => format!("{},{}", 4, v),
            Instruction::OUT { combo } => format!("{},{}", 5, combo),
            Instruction::BDV { combo } => format!("{},{}", 6, combo),
            Instruction::CDV { combo } => format!("{},{}", 7, combo),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "
            Register A: 729
            Register B: 0
            Register C: 0

            Program: 0,1,5,4,3,0
        ";
        assert_eq!(part1(INPUT), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "
            Register A: 30899381
            Register B: 0
            Register C: 0

            Program: 2,4,1,1,7,5,4,0,0,3,1,6,5,5,3,0
        ";
        assert_eq!(part2(INPUT), "247839653009594");
    }
}
