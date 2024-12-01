use std::collections::HashMap;

pub fn puzzle_1(input: &str) -> usize {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    for line in input.lines() {
        let pair: Vec<usize> = line
            .trim()
            .split_whitespace()
            .filter_map(|line| line.parse::<usize>().ok())
            .collect();
        left.push(pair[0]);
        right.push(pair[1]);
    }
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (left, right)| acc + left.abs_diff(*right))
}

pub fn puzzle_2(input: &str) -> usize {
    let (mut left, mut right) = (HashMap::new(), HashMap::new());
    for line in input.lines() {
        let pair: Vec<usize> = line
            .trim()
            .split_whitespace()
            .filter_map(|line| line.parse::<usize>().ok())
            .collect();
        left.entry(pair[0])
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        right
            .entry(pair[1])
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    left.iter().fold(0, |acc, (key, value)| {
        acc + key * value * right.get(key).unwrap_or(&0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input: String = "3   4
4   3
2   5
1   3
3   9
3   3"
            .to_owned();
        assert_eq!(puzzle_1(&input), 11);
    }

    #[test]
    fn test_2() {
        let input: String = "3   4 
4   3
2   5
1   3
3   9
3   3"
            .to_owned();
        assert_eq!(puzzle_2(&input), 31);
    }
}
