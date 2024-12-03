// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
pub fn puzzle_1(input: &str) -> String {
    input
        .lines()
        .fold(0, |acc, line| {
            let matches: Vec<_> = line.match_indices("mul(").map(|(i, _)| i).collect();
            acc + matches.iter().fold(0, |acc, mul_match| {
                acc + find_mul_values_and_calc(&line, *mul_match)
            })
        })
        .to_string()
}

fn find_mul_values_and_calc(line: &str, mul_match: usize) -> usize {
    let parenthisis = line[mul_match + 4..].find(')');
    if let Some(end) = parenthisis {
        let ss = &line[(mul_match + 4)..(mul_match + end + 4)];
        if let Some((left, right)) = ss
            .split_once(',')
            .map(|(left, right)| (left.parse::<usize>(), right.parse::<usize>()))
        {
            match (left, right) {
                (Ok(left), Ok(right)) => return right * left,
                _ => (),
            }
        }
    }
    return 0;
}

// Solution has
// Time complexity: O(n*log(n))
// Space complexity: O(n)
pub fn puzzle_2(input: &str) -> String {
    let mut result = 0;
    let mut is_active = true;
    for line in input.lines() {
        let matches: Vec<_> = line.match_indices("mul(").map(|(i, _)| i).collect();
        let doer: Vec<(usize, bool)> = line.match_indices("do()").map(|(i, _)| (i, true)).collect();
        let dont: Vec<(usize, bool)> = line
            .match_indices("don't()")
            .map(|(i, _)| (i, false))
            .collect();
        let mut ranges = doer;
        for val in dont {
            ranges.push(val);
        }

        ranges.sort_unstable_by_key(|i| i.0);

        for mul_match in matches {
            is_active = find_do_dont_range(&ranges, mul_match, is_active);
            if is_active {
                result += find_mul_values_and_calc(&line, mul_match);
            }
        }
    }
    result.to_string()
}

fn find_do_dont_range(ranges: &[(usize, bool)], active_idx: usize, is_active: bool) -> bool {
    let mut active = is_active;
    for range in ranges {
        if range.0 < active_idx {
            active = range.1;
        } else {
            break;
        }
    }
    active
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "161");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "48");
    }
}
