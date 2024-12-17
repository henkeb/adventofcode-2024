use regex::Regex;

pub fn puzzle_1(input: &str) -> String {
    let (a, b, c, p): (usize, usize, usize, Vec<usize>) = handle_input(input);
    let mut computer = Computer::new(a, b, c, p);
    computer.solve().join(",")
}

// First digit changes every increment of A
// Second digit changes every 8th increment of A
// Third digit changes every 64th increment of A
// Fourth digit changes every 512th increment of A
// and so on.
// Program: 2,4,1,5,7,5,1,6,0,3,4,2,5,5,3,0

pub fn puzzle_2(input: &str) -> String {
    let (_, b, c, p) = handle_input(input);

    let mut computer = Computer::new(0, b, c, p.clone());
    let program_input: Vec<String> = p.iter().map(|val| val.to_string()).collect();

    let mut factors = vec![0; program_input.len()];

    loop {
        let mut a = 0;
        for (i, factor) in factors.iter().enumerate() {
            a += 8usize.pow(i as u32) * *factor as usize;
        }
        computer.reset();
        computer.a = a;
        let program_output = computer.solve();
        if program_output == program_input {
            return a.to_string();
        }
        for i in (0..program_input.len()).rev() {
            if program_output.len() < i {
                factors[i] += 1;
                break;
            }
            if program_output[i] != program_input[i] {
                factors[i] += 1;
                break;
            }
        }
    }
}

fn handle_input(input: &str) -> (usize, usize, usize, Vec<usize>) {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut p = Vec::new();

    let re = Regex::new(r"(\d+)").unwrap();

    for (i, line) in input.lines().enumerate() {
        for (_, [x]) in re.captures_iter(line).map(|c| c.extract()) {
            if i == 0 {
                a = x.parse::<usize>().unwrap();
            } else if i == 1 {
                b = x.parse::<usize>().unwrap();
            } else if i == 2 {
                c = x.parse::<usize>().unwrap();
            } else if i >= 3 {
                p.push(x.parse::<usize>().unwrap());
            }
        }
    }
    (a, b, c, p)
}

struct Computer {
    a: usize,
    b: usize,
    c: usize,
    p: Vec<usize>,
    instruction_pointer: usize,
    output: Vec<String>,
}

impl Computer {
    fn new(a: usize, b: usize, c: usize, p: Vec<usize>) -> Self {
        Computer {
            a,
            b,
            c,
            p,
            instruction_pointer: 0,
            output: vec![],
        }
    }

    fn reset(&mut self) {
        self.b = 0;
        self.c = 0;
        self.instruction_pointer = 0;
        self.output = Vec::new();
    }

    fn do_instruction(&mut self, instruction: usize, literal: usize) {
        let combo_operand = match literal {
            0..=3 => literal,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Wrong operand"),
        };
        match instruction {
            0 => self.a /= 2_usize.pow(combo_operand as u32),
            1 => self.b ^= literal,
            2 => self.b = combo_operand % 8,
            3 => {
                if self.a != 0 {
                    self.instruction_pointer = literal;
                    return;
                }
            }
            4 => self.b ^= self.c,
            5 => self.output.push((combo_operand % 8).to_string()),
            6 => self.b = self.a / 2_u32.pow(combo_operand as u32) as usize,
            7 => self.c = self.a / 2_u32.pow(combo_operand as u32) as usize,
            _ => (),
        }
        self.instruction_pointer += 2;
    }

    fn solve(&mut self) -> Vec<String> {
        while self.instruction_pointer < (self.p.len() - 1) {
            self.do_instruction(
                self.p[self.instruction_pointer],
                self.p[self.instruction_pointer + 1],
            );
        }
        self.output.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT2), "117440");
    }
}
