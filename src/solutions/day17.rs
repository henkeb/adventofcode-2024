use regex::Regex;

pub fn puzzle_1(input: &str) -> String {
    let (a, b, c, p): (usize, usize, usize, Vec<usize>) = handle_input(input);
    let mut computer = Computer::new(a, b, c, p);
    computer.solve().join(",")
}

pub fn puzzle_2(input: &str) -> String {
    // let _ = handle_input(input);
    "Not implemented yet!".to_string()
}

fn handle_input(input: &str) -> (usize, usize, usize, Vec<usize>) {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut p = Vec::new();

    let regex = Regex::new(r"(\d+)").unwrap();

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            a = regex
                .captures(&line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
        } else if i == 1 {
            b = regex
                .captures(&line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
        } else if i == 2 {
            c = regex
                .captures(&line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
        } else if i == 4 {
            p = regex
                .captures_iter(&line)
                .map(|cap| cap.get(0).unwrap().as_str().parse().unwrap())
                .collect();
        }
    }
    (a, b, c, p)
}

struct Computer {
    a: usize,
    b: usize,
    c: usize,
    p: Vec<usize>,
    active_instruction: usize,
    output: Vec<String>,
}

impl Computer {
    fn new(a: usize, b: usize, c: usize, p: Vec<usize>) -> Self {
        Computer {
            a,
            b,
            c,
            p,
            active_instruction: 0,
            output: vec![],
        }
    }

    fn do_instruction(&mut self, instruction: usize, literal: usize) {
        let combo_operand = match literal {
            0..=3 => literal,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("too high combo operand"),
        };
        let mut did_jump = false;
        match instruction {
            0 => self.a /= 2_usize.pow(combo_operand as u32),
            1 => self.b ^= literal,
            2 => self.b = combo_operand % 8,
            3 => {
                if self.a != 0 {
                    self.active_instruction = literal;
                    did_jump = true;
                }
            }
            4 => self.b ^= self.c,
            5 => self.output.push((combo_operand % 8).to_string()),
            6 => self.b = self.a / 2_u32.pow(combo_operand as u32) as usize,
            7 => self.c = self.a / 2_u32.pow(combo_operand as u32) as usize,
            _ => panic!("wrong instruction"),
        }
        if instruction == 5 {
            dbg!(&self.output);
        }
        if !did_jump {
            self.active_instruction += 2;
        }
    }

    fn solve(&mut self) -> Vec<String> {
        while self.active_instruction < (self.p.len() - 1) {
            self.do_instruction(
                self.p[self.active_instruction],
                self.p[self.active_instruction + 1],
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

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "");
    }
}
