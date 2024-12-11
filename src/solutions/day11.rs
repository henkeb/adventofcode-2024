use std::collections::HashMap;

pub fn puzzle_1(input: &str) -> String {
    (0..25)
        .fold(handle_input(input), |stones, _| update_stones(&stones))
        .values()
        .sum::<usize>()
        .to_string()
}

pub fn puzzle_2(input: &str) -> String {
    (0..75)
        .fold(handle_input(input), |stones, _| update_stones(&stones))
        .values()
        .sum::<usize>()
        .to_string()
}

fn update_stones(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones: HashMap<usize, usize> = HashMap::with_capacity(stones.len());
    let modify_stones = |stones: &mut HashMap<usize, usize>, new_stone: &usize, count: &usize| {
        stones
            .entry(*new_stone)
            .and_modify(|prev_stone| *prev_stone += count)
            .or_insert(*count);
    };
    for (stone, count) in stones.iter() {
        if *stone == 0 {
            modify_stones(&mut new_stones, &1, count);
        } else if (stone.ilog10() + 1) & 1 == 0 {
            let stone_str = stone.to_string();
            let (left, right) = stone_str.split_at(stone_str.len() / 2);
            if let (Some(left), Some(right)) = (left.parse().ok(), right.parse().ok()) {
                modify_stones(&mut new_stones, &left, count);
                modify_stones(&mut new_stones, &right, count);
            }
        } else {
            modify_stones(&mut new_stones, &(*stone * 2024), count);
        }
    }
    new_stones
}

fn handle_input(input: &str) -> HashMap<usize, usize> {
    let mut output: HashMap<usize, usize> = HashMap::new();
    input
        .trim()
        .split_whitespace()
        .filter_map(|stone| stone.parse::<usize>().ok())
        .for_each(|stone| {
            output
                .entry(stone)
                .and_modify(|prev_stone| *prev_stone += 1)
                .or_insert(1);
        });
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "55312");
    }
}
