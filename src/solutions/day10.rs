use std::collections::HashMap;

const DIRECTION: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
pub fn puzzle_1(input: &str) -> String {
    let (map, max_len) = handle_input(input);
    let mut count = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, num) in row.iter().enumerate() {
            if *num == 0 {
                count += dfs(&map, &max_len, &(x as isize, y as isize), false);
            }
        }
    }

    count.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let (map, max_len) = handle_input(input);
    let mut count = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, num) in row.iter().enumerate() {
            if *num == 0 {
                count += dfs(&map, &max_len, &(x as isize, y as isize), true);
            }
        }
    }

    count.to_string()
}

fn dfs(
    map: &Vec<Vec<usize>>,
    max_len: &(isize, isize),
    start_pos: &(isize, isize),
    unique_trainheads: bool,
) -> usize {
    let check_bounds = |a: (isize, isize), b: (isize, isize)| -> bool {
        if a.0 >= 0 && a.0 < b.0 {
            if a.1 >= 0 && a.1 < b.1 {
                return true;
            }
        }
        false
    };
    let mut found_tippeditops: HashMap<(isize, isize), usize> = HashMap::new();
    let mut queue: Vec<((isize, isize), usize)> = Vec::new();
    queue.push(((*start_pos), 0));
    while let Some((pos, gradient)) = queue.pop() {
        for dir in DIRECTION.iter() {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if !check_bounds(new_pos, *max_len) {
                continue;
            }

            if map[new_pos.1 as usize][new_pos.0 as usize] != gradient + 1 {
                continue;
            }

            if map[new_pos.1 as usize][new_pos.0 as usize] == 9 {
                found_tippeditops
                    .entry(new_pos)
                    .and_modify(|val| *val += 1)
                    .or_insert(1);
            } else {
                queue.push((new_pos, gradient + 1));
            }
        }
    }
    if unique_trainheads {
        found_tippeditops.iter().map(|(_, value)| value).sum()
    } else {
        found_tippeditops.iter().count()
    }
}

fn handle_input(input: &str) -> (Vec<Vec<usize>>, (isize, isize)) {
    let mut output = Vec::new();
    for (y, row) in input.lines().enumerate() {
        output.push(vec![]);
        for (_, ch) in row.chars().enumerate() {
            if ch == '.' {
                output[y].push(11);
            } else {
                output[y].push(ch as usize - '0' as usize);
            }
        }
    }
    let x_len = output[0].len() as isize;
    let y_len = output.len() as isize;
    (output, (x_len, y_len))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0123
1234
8765
9876";

    #[test]
    fn test_1a() {
        assert_eq!(puzzle_1(&INPUT), "1");
    }

    const INPUT2: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_1b() {
        assert_eq!(puzzle_1(&INPUT2), "36");
    }

    #[test]
    fn test_2a() {
        assert_eq!(
            puzzle_2(
                ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."
            ),
            "3"
        );
    }

    #[test]
    fn test_2b() {
        assert_eq!(puzzle_2(&INPUT2), "81");
    }
}
