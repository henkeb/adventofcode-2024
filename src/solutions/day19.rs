use std::collections::HashMap;

pub fn puzzle_1(input: &str) -> String {
    let (stripe_colours, towel_combinations) = handle_input(input);
    let mut memo = HashMap::new();
    towel_combinations
        .iter()
        .filter(|towel_combination| {
            count_possible_combinations(towel_combination, &stripe_colours, &mut memo) > 0
        })
        .collect::<Vec<_>>()
        .len()
        .to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let (stripe_colours, towel_combinations) = handle_input(input);
    let mut memo = HashMap::new();
    towel_combinations
        .iter()
        .map(|towel_combination| {
            count_possible_combinations(towel_combination, &stripe_colours, &mut memo)
        })
        .sum::<usize>()
        .to_string()
}

fn count_possible_combinations(
    towel_combination: &str,
    stripe_colours: &Vec<String>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if let Some(seen) = memo.get(towel_combination) {
        return *seen;
    }

    if towel_combination.trim().is_empty() {
        return 1;
    }

    let mut count = 0;
    for colour in stripe_colours.iter() {
        if towel_combination.starts_with(colour) {
            count += count_possible_combinations(
                &towel_combination[colour.len()..],
                stripe_colours,
                memo,
            );
        }
    }

    memo.entry(towel_combination.to_string())
        .and_modify(|combinations| *combinations += count)
        .or_insert(count);
    count
}

fn handle_input(input: &str) -> (Vec<String>, Vec<String>) {
    let (stripe_colours, towel_combinations) = input.split_once("\n\n").unwrap();
    (
        stripe_colours.split(", ").map(String::from).collect(),
        towel_combinations.lines().map(String::from).collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "6");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "16");
    }
}
