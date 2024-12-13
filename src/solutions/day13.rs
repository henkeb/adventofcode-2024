use regex::Regex;

pub fn puzzle_1(input: &str) -> String {
    let claw_games = handle_input(input, false);

    let mut output = 0;
    for claw_game in claw_games.iter() {
        let mut button_presses = linear_solutions(
            claw_game.button_a.0,
            claw_game.button_b.0,
            claw_game.prize.0,
        );
        button_presses.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        for presses in button_presses.iter() {
            if presses.0 * claw_game.button_a.1 + presses.1 * claw_game.button_b.1
                == claw_game.prize.1
            {
                output += presses.0 * 3 + presses.1 * 1;
                break;
            }
        }
    }
    output.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let claw_games = handle_input(input, true);
    let mut output = 0;
    for claw_game in claw_games.iter() {
        let mut button_presses = linear_solutions(
            claw_game.button_a.0,
            claw_game.button_b.0,
            claw_game.prize.0,
        );
        button_presses.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        for presses in button_presses.iter() {
            if presses.0 * claw_game.button_a.1 + presses.1 * claw_game.button_b.1
                == claw_game.prize.1
            {
                output += presses.0 * 3 + presses.1 * 1;
                break;
            }
        }
    }
    output.to_string()
}
fn linear_solutions(n: usize, m: usize, constant: usize) -> Vec<(usize, usize)> {
    let mut solutions = Vec::new();
    for y in 0..100000000 {
        if !(constant as isize - m as isize * y as isize).is_positive() {
            continue;
        }
        let remainder = (constant - m * y) % n;
        if remainder == 0 {
            solutions.push(((constant - m * y) / n, y));
        }
    }
    solutions
}

fn handle_input(input: &str, has_conversion_error: bool) -> Vec<ClawGame> {
    let mut output = Vec::new();
    let re = Regex::new(r"(\d*), Y\+(\d*)|(\d+), Y=(\d*)").unwrap();
    input.split("\n\n").for_each(|group| {
        let mut results = vec![];
        for ele in group.lines() {
            for (_, [x, y]) in re.captures_iter(ele).map(|c| c.extract()) {
                results.push((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
            }
        }
        let mut prize = (results[2].clone().0, results[2].clone().1);
        if has_conversion_error {
            prize = (prize.0 + 10000000000000, prize.1 + 10000000000000);
        }
        output.push(ClawGame {
            button_a: (results[0].clone().0, results[0].clone().1),
            button_b: (results[1].clone().0, results[1].clone().1),
            prize,
        });
    });
    output
}

#[derive(Debug)]
struct ClawGame {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "480");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "");
    }
}
