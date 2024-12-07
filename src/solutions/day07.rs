use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    let mut sums_nums: Vec<(usize, Vec<usize>)> = Vec::new();
    for line in input.lines() {
        if let Some(separator) = line.find(':') {
            let (a, b) = line.split_at(separator);
            let sum: usize = a.parse().unwrap();
            let nums = b
                .trim()
                .split_whitespace()
                .filter_map(|val| val.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            sums_nums.push((sum, nums));
        }
    }
    sums_nums
}

// Solution has
// Time complexity: O(?)
// Space complexity: O(?)
pub fn puzzle_1(input: &str) -> String {
    let sums_nums = parse_input(input);
    sums_nums
        .iter()
        .fold(0, |acc, (sum, nums)| {
            let num_operators = nums.len() - 1;
            let combinations = 2_usize.pow(num_operators as u32);
            for combination in 0..combinations {
                let mut sums = nums[0];
                for i in 0..num_operators {
                    if combination >> i & 1 == 1 {
                        sums *= nums[i + 1];
                    } else {
                        sums += nums[i + 1];
                    }
                }
                if *sum == sums {
                    return acc + sums;
                }
            }
            acc
        })
        .to_string()
}

// Solution has
// Time complexity: O(?)
// Space complexity: O(?)
pub fn puzzle_2(input: &str) -> String {
    let sums_nums = parse_input(input);

    let conc = |a: usize, b: usize| str::parse::<usize>(&format!("{a}{b}")).unwrap();

    sums_nums
        .iter()
        .fold(0, |acc, (target, nums)| {
            let calculations = nums
                .iter()
                .skip(1)
                .fold(HashSet::from([nums[0]]), |acc, num| {
                    acc.into_iter()
                        .flat_map(|prev| {
                            let mut outp: Vec<usize> = Vec::new();
                            let add = prev + num;
                            let mul = prev * num;
                            let con = conc(prev, *num);
                            if add <= *target {
                                outp.push(add);
                            }
                            if mul <= *target {
                                outp.push(mul);
                            }
                            if con <= *target {
                                outp.push(con);
                            }
                            outp
                        })
                        .collect()
                });
            if calculations.contains(&target) {
                acc + target
            } else {
                acc
            }
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "3749");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "11387");
    }
}
