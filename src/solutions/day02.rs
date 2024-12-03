// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
pub fn puzzle_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .filter_map(|val| val.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .fold(
            0,
            |acc, line| {
                if is_line_safe(&line) {
                    acc + 1
                } else {
                    acc
                }
            },
        )
        .to_string()
}

// Solution has
// Time complexity: O(n*m)
// Space complexity: O(1)
pub fn puzzle_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .filter_map(|val| val.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .fold(0, |acc, line| {
            for i in 0..line.len() {
                let first_part = &line[0..i];
                let second_part = &line[i + 1..line.len()];
                if is_line_safe(&[first_part, second_part].concat()) {
                    return acc + 1;
                }
            }
            acc
        })
        .to_string()
}

fn is_line_safe(line: &[usize]) -> bool {
    let mut _is_ascending = false;
    match line[0].cmp(&line[1]) {
        std::cmp::Ordering::Equal => return false,
        std::cmp::Ordering::Less => _is_ascending = true,
        std::cmp::Ordering::Greater => _is_ascending = false,
    }

    for i in 1..line.len() {
        match line[i - 1].cmp(&line[i]) {
            std::cmp::Ordering::Less if _is_ascending && line[i] - line[i - 1] <= 3 => continue,
            std::cmp::Ordering::Greater if !_is_ascending && line[i - 1] - line[i] <= 3 => continue,
            _ => {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "2");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "4");
    }
}
