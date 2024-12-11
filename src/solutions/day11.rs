use std::collections::HashMap;

pub fn puzzle_1(input: &str) -> String {
    let mut stones = handle_input(input);

    for _ in 0..25 {
        stones = update_stones(&stones);
    }

    stones.values().sum::<usize>().to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mut stones = handle_input(input);

    for _ in 0..75 {
        stones = update_stones(&stones);
    }

    stones.values().sum::<usize>().to_string()
}

fn update_stones(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones: HashMap<usize, usize> = HashMap::with_capacity(stones.len());
    for (stone, count) in stones.iter() {
        if *stone == 0 {
            new_stones
                .entry(1)
                .and_modify(|prev| *prev += count)
                .or_insert(*count);
        } else if count_digits(stone) & 1 == 0 {
            let stone_str = stone.to_string();
            let (left, right) = stone_str.split_at(stone_str.len() / 2);
            new_stones
                .entry(left.parse().unwrap())
                .and_modify(|prev| *prev += count)
                .or_insert(*count);
            new_stones
                .entry(right.parse().unwrap())
                .and_modify(|prev| *prev += count)
                .or_insert(*count);
        } else {
            new_stones
                .entry(*stone * 2024)
                .and_modify(|prev| *prev += count)
                .or_insert(*count);
        }
    }
    new_stones
}

fn count_digits(number: &usize) -> usize {
    let mut digits = 0;
    let mut temp_num = number.clone();
    while temp_num > 0 {
        digits += 1;
        temp_num /= 10;
    }
    digits
}

fn handle_input(input: &str) -> HashMap<usize, usize> {
    input
        .trim()
        .split_whitespace()
        .map(|val| (val.parse().unwrap(), 1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "55312");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "");
    }
}
