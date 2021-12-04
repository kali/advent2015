use std::fs;

fn main() {
    let program: Vec<String> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .filter(|s| s.len() > 0)
        .map(|s| s.to_string())
        .collect();
    for &start in &[0, 1] {
        let mut pc = 0;
        let mut regs = [start, 0];
        loop {
            let op = &program[pc];
            dbg!(op);
            let opcode = &op[0..3];
            let operands: Vec<&str> = op[4..].split(", ").collect();
            match opcode {
                "hlf" => regs[(operands[0].as_bytes()[0] - b'a') as usize] /= 2,
                "inc" => regs[(operands[0].as_bytes()[0] - b'a') as usize] += 1,
                "tpl" => regs[(operands[0].as_bytes()[0] - b'a') as usize] *= 3,
                "jie" => {
                    let value = regs[(operands[0].as_bytes()[0] - b'a') as usize];
                    if value % 2 == 0 {
                        pc = (pc as isize + operands[1].parse::<isize>().unwrap() - 1) as usize;
                    }
                }
                "jio" => {
                    let value = regs[(operands[0].as_bytes()[0] - b'a') as usize];
                    if value == 1 {
                        pc = (pc as isize + operands[1].parse::<isize>().unwrap() - 1) as usize;
                    }
                }
                "jmp" => {
                    pc = (pc as isize + operands[0].parse::<isize>().unwrap() - 1) as usize;
                }
                _ => unimplemented!(),
            }
            pc += 1;
            if pc >= program.len() {
                break;
            }
        }
        dbg!(regs);
    }
}
