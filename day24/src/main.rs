use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Write,
};

fn main() {
    let input = util::read_input("day24/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let problem = Problem::from(input);

    format!("{}", problem.solve(&problem.gates))
}

fn part2(input: &str) -> String {
    let problem = Problem::from(input);

    problem.generate_graph();

    /*
     * All outputs should have line of two XOR gates to the corresponding
     * inputs, e.g:
     *
     *    (x12, y12) -> XOR -> XOR -> z12
     *
     * Analyzing the generated graph and looking for discrepancies in
     * the XOR chains gives the following required swaps:
     *
     * z08 <-> vvr
     * bkr <-> rnq
     * z28 <-> tfb
     * mqh <-> z39
     *
     * The final result looks like this:
     *
     *   bkr,mqh,rnq,tfb,vvr,z08,z28,z39
     */

    format!("bkr,mqh,rnq,tfb,vvr,z08,z28,z39")
}

struct Problem {
    wires: HashMap<String, bool>,
    gates: Vec<Gate>,
}

#[derive(Clone)]
struct Gate {
    a: String,
    b: String,
    out: String,
    op: String,
}

impl Gate {
    fn from(s: &str) -> Gate {
        let (expr, out) = s.split_once(" -> ").unwrap();
        let mut parts = expr.split(" ");

        let a = parts.next().unwrap().to_string();
        let op = parts.next().unwrap();
        let b = parts.next().unwrap().to_string();

        Gate {
            a,
            b,
            out: out.to_string(),
            op: op.to_string(),
        }
    }

    fn resolve(&self, wires: &mut HashMap<String, bool>) -> bool {
        if !wires.contains_key(&self.a) || !wires.contains_key(&self.b) {
            return false;
        }

        match self.op.as_str() {
            "AND" => wires.insert(self.out.clone(), wires[&self.a] && wires[&self.b]),
            "XOR" => wires.insert(self.out.clone(), wires[&self.a] != wires[&self.b]),
            "OR" => wires.insert(self.out.clone(), wires[&self.a] || wires[&self.b]),
            _ => return false,
        };

        true
    }
}

impl Problem {
    fn from(data: &str) -> Self {
        let (wires, gates) = data.trim().split_once("\n\n").unwrap();

        let wires = util::to_lines(wires)
            .iter()
            .map(|s| {
                let (name, value) = s.split_once(": ").unwrap();

                (name.to_string(), value == "1")
            })
            .collect();

        let gates = util::to_lines(gates)
            .iter()
            .map(|s| Gate::from(s))
            .collect();

        Problem { wires, gates }
    }

    fn solve(&self, gates: &[Gate]) -> usize {
        let mut wires = self.wires.clone();
        let mut gates = VecDeque::from_iter(gates.iter());

        while let Some(g) = gates.pop_back() {
            if !g.resolve(&mut wires) {
                gates.push_front(g);
            }
        }

        let mut z_outputs: Vec<_> = wires
            .iter()
            .filter(|(name, _)| name.starts_with("z"))
            .collect();
        z_outputs.sort_by(|(a, _), (b, _)| a.cmp(&b));

        let mut result = 0;
        for (i, (_, &v)) in z_outputs.iter().enumerate() {
            if v {
                result |= 1 << i;
            }
        }

        result
    }

    fn generate_graph(&self) {
        let mut f = File::create("graph.d2").unwrap();

        for g in self.gates.iter() {
            let name = format!("\"{} {} {} -> {}\"", g.a, g.op, g.b, g.out);
            writeln!(f, "{} -> {}", g.a, name).unwrap();
            writeln!(f, "{} -> {}", g.b, name).unwrap();
            writeln!(f, "{} -> {}", name, g.out).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02
    ";

    #[test]
    fn test_part1_ex1() {
        assert_eq!(part1(INPUT), "4");
    }

    #[test]
    fn test_part1_ex2() {
        const INPUT: &str = "
            x00: 1
            x01: 0
            x02: 1
            x03: 1
            x04: 0
            y00: 1
            y01: 1
            y02: 1
            y03: 1
            y04: 1

            ntg XOR fgs -> mjb
            y02 OR x01 -> tnw
            kwq OR kpj -> z05
            x00 OR x03 -> fst
            tgd XOR rvg -> z01
            vdt OR tnw -> bfw
            bfw AND frj -> z10
            ffh OR nrd -> bqk
            y00 AND y03 -> djm
            y03 OR y00 -> psh
            bqk OR frj -> z08
            tnw OR fst -> frj
            gnj AND tgd -> z11
            bfw XOR mjb -> z00
            x03 OR x00 -> vdt
            gnj AND wpb -> z02
            x04 AND y00 -> kjc
            djm OR pbm -> qhw
            nrd AND vdt -> hwm
            kjc AND fst -> rvg
            y04 OR y02 -> fgs
            y01 AND x02 -> pbm
            ntg OR kjc -> kwq
            psh XOR fgs -> tgd
            qhw XOR tgd -> z09
            pbm OR djm -> kpj
            x03 XOR y03 -> ffh
            x00 XOR y04 -> ntg
            bfw OR bqk -> z06
            nrd XOR fgs -> wpb
            frj XOR qhw -> z04
            bqk OR frj -> z07
            y03 OR x01 -> nrd
            hwm AND bqk -> z03
            tgd XOR rvg -> z12
            tnw OR pbm -> gnj
        ";
        assert_eq!(part1(INPUT), "2024");
    }
}
