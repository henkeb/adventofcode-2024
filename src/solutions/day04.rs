const DIRECTION: [(isize, isize); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

// Solution has
// Time complexity: O(n*m)
// Space complexity: O(1)
pub fn puzzle_1(input: &str) -> String {
    let mut count: usize = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == 'X' {
                count += search((i, j), input);
            }
        }
    }
    count.to_string()
}

fn check_bounds(
    position: &(usize, usize),
    dir: &(isize, isize),
    input: &str,
) -> Option<(usize, usize)> {
    let row: isize = position.0 as isize + dir.0;
    let col: isize = position.1 as isize + dir.1;

    if !row.is_negative() && !col.is_negative() {
        let row = row as usize;
        let col = col as usize;
        if row < input.lines().count() && col < input.lines().next().unwrap().len() {
            return Some((row as usize, col as usize));
        }
    }
    None
}

fn get_char(input: &str, position: &(usize, usize)) -> Option<char> {
    if let Some(line) = input.lines().nth(position.0) {
        line.chars().nth(position.1)
    } else {
        None
    }
}

fn search(position: (usize, usize), input: &str) -> usize {
    let mut count = 0;
    for dir in DIRECTION.iter() {
        if let Some(pos_m) = check_bounds(&position, dir, input) {
            if let Some(next_ch) = get_char(input, &pos_m) {
                if next_ch == 'M' {
                    if let Some(pos_a) = check_bounds(&pos_m, dir, input) {
                        if let Some(next_ch) = get_char(input, &pos_a) {
                            if next_ch == 'A' {
                                if let Some(pos_s) = check_bounds(&pos_a, dir, input) {
                                    if let Some(next_ch) = get_char(input, &pos_s) {
                                        if next_ch == 'S' {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    count
}
const DIRECTION_DIAG: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

// Solution has
// Time complexity: O(n*m)
// Space complexity: O(1)
pub fn puzzle_2(input: &str) -> String {
    let mut count: usize = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == 'A' {
                count += search_x((i, j), input);
            }
        }
    }
    count.to_string()
}

fn search_x(position: (usize, usize), input: &str) -> usize {
    let mut ch_arr: [char; 4] = ['0'; 4];
    for (i, dir) in DIRECTION_DIAG.iter().enumerate() {
        if let Some(pos) = check_bounds(&position, dir, input) {
            if let Some(ch) = get_char(input, &pos) {
                ch_arr[i] = ch;
            }
        }
    }
    if ch_arr[0] == 'M' && ch_arr[3] == 'S' || ch_arr[0] == 'S' && ch_arr[3] == 'M' {
        if ch_arr[1] == 'M' && ch_arr[2] == 'S' || ch_arr[1] == 'S' && ch_arr[2] == 'M' {
            return 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "18");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "9");
    }
}
