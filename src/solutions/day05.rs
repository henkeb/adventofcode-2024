use std::collections::{HashMap, HashSet, VecDeque};

fn handle_input(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut parts = input.split("\n\n");
    let rules = parts.next().unwrap();
    let updates_raw = parts.next().unwrap();

    let mut rules_map_number_before: HashMap<usize, Vec<usize>> = HashMap::new();

    rules
        .lines()
        .map(|line| line.split('|').filter_map(|val| val.parse::<usize>().ok()))
        .map(|mut left_right| (left_right.next().unwrap(), left_right.next().unwrap()))
        .for_each(|(before, after)| {
            rules_map_number_before
                .entry(before)
                .and_modify(|v| v.push(after))
                .or_insert(vec![after]);
        });

    let mut updates: Vec<Vec<usize>> = Vec::new();

    for raw_update in updates_raw.lines() {
        updates.push(
            raw_update
                .split(',')
                .filter_map(|val| val.parse::<usize>().ok())
                .collect(),
        );
    }

    (rules_map_number_before, updates)
}

// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
pub fn puzzle_1(input: &str) -> String {
    let (rules_map_number_before, updates) = handle_input(input);

    let mut count = 0;
    for update in updates.iter() {
        let mut handled_values: HashSet<usize> = HashSet::new();
        let mut is_update_ok = true;
        for value in update.iter() {
            if let Some(before_list) = rules_map_number_before.get(&value) {
                for before in before_list.iter() {
                    if handled_values.contains(&before) {
                        is_update_ok = false;
                        break;
                    }
                }
            }
            if !is_update_ok {
                break;
            }
            handled_values.insert(*value);
        }
        if is_update_ok {
            count += update[update.len() / 2];
        }
    }
    count.to_string()
}

// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
pub fn puzzle_2(input: &str) -> String {
    let (rules_map_number_before, updates) = handle_input(input);

    let mut failed_updates: Vec<Vec<usize>> = Vec::new();

    let mut count = 0;
    for update in updates.iter() {
        let mut handled_values: HashSet<usize> = HashSet::new();
        let mut is_update_ok = true;
        for value in update.iter() {
            if let Some(before_list) = rules_map_number_before.get(&value) {
                for before in before_list.iter() {
                    if handled_values.contains(&before) {
                        failed_updates.push(update.clone());
                        is_update_ok = false;
                        break;
                    }
                }
            }
            if !is_update_ok {
                break;
            }
            handled_values.insert(*value);
        }
    }

    for failed_update in failed_updates.iter() {
        let mut ordered: VecDeque<usize> = VecDeque::new();
        for _ in 0..failed_update.len() {
            for val in failed_update.iter() {
                if !ordered.contains(val) {
                    if let Some(before_list) = rules_map_number_before.get(val) {
                        let mut needs_before: HashSet<usize> = HashSet::new();
                        for before in before_list.iter() {
                            if failed_update.contains(before) {
                                needs_before.insert(*before);
                            }
                        }
                        let mut all_before = true;
                        for before in needs_before.iter() {
                            if !ordered.contains(before) {
                                all_before = false;
                                break;
                            }
                        }
                        if all_before {
                            ordered.push_front(*val);
                        }
                    } else {
                        ordered.push_front(*val);
                    }
                }
            }
        }
        count += ordered[ordered.len() / 2];
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "143");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "123");
    }
}
