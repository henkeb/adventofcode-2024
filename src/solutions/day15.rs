pub fn puzzle_1(input: &str) -> String {
    let (mut map, max_map_len, commands) = handle_input(input);
    let mut position = get_starting_position(&map);

    for command in commands.iter() {
        let new_position = (
            position.0 + command.value().0,
            position.1 + command.value().1,
        );
        if !check_bounds(&new_position, &max_map_len) {
            continue;
        }
        match map[new_position.1 as usize][new_position.0 as usize] {
            '.' => (),
            '#' => continue,
            'O' => {
                if !move_boxes_if_possible(&mut map, command, &max_map_len, &new_position) {
                    continue;
                }
            }
            _ => {
                dbg!(map[new_position.1 as usize][new_position.0 as usize]);
                panic!("Wrong char");
            }
        }
        map[new_position.1 as usize][new_position.0 as usize] = '@';
        map[position.1 as usize][position.0 as usize] = '.';
        position = new_position;
    }
    calculate_gps_coordinates(&map).to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let _ = handle_input(input);
    "Not implemented yet!".to_string()
}

fn calculate_gps_coordinates(map: &Vec<Vec<char>>) -> usize {
    let mut output = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == 'O' {
                output += y * 100 + x;
            }
        }
    }
    output
}

fn move_boxes_if_possible(
    map: &mut Vec<Vec<char>>,
    command: &Direction,
    max: &(isize, isize),
    new_position: &(isize, isize),
) -> bool {
    let mut find_empty_position = new_position.clone();
    while find_empty_position.0 > 0
        && find_empty_position.0 < (max.0 - 1)
        && find_empty_position.1 > 0
        && find_empty_position.1 < (max.1 - 1)
    {
        find_empty_position = (
            find_empty_position.0 + command.value().0,
            find_empty_position.1 + command.value().1,
        );

        match map[find_empty_position.1 as usize][find_empty_position.0 as usize] {
            '#' => return false,
            '.' => {
                map[find_empty_position.1 as usize][find_empty_position.0 as usize] = 'O';
                map[new_position.1 as usize][new_position.0 as usize] = '.';
                return true;
            }
            _ => continue,
        }
    }
    false
}

fn check_bounds(point: &(isize, isize), max: &(isize, isize)) -> bool {
    if point.0 >= 0 && point.0 < max.0 {
        if point.1 >= 0 && point.1 < max.1 {
            return true;
        }
    }
    false
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn value(&self) -> (isize, isize) {
        match *self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}
fn get_starting_position(map: &Vec<Vec<char>>) -> (isize, isize) {
    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == '@' {
                return (x as isize, y as isize);
            }
        }
    }
    (0, 0)
}
fn handle_input(input: &str) -> (Vec<Vec<char>>, (isize, isize), Vec<Direction>) {
    let (map, commands) = input.split_once("\n\n").unwrap();
    let mut map_output: Vec<Vec<char>> = Vec::new();
    for (i, line) in map.lines().enumerate() {
        map_output.push(vec![]);
        for ch in line.chars() {
            map_output[i].push(ch);
        }
    }
    let max_map_len = (map_output[0].len() as isize, map_output.len() as isize);

    let mut commands_output: Vec<Direction> = Vec::with_capacity(commands.chars().count());

    for line in commands.lines() {
        for ch in line.chars() {
            commands_output.push(match ch {
                '^' => Direction::Up,
                '<' => Direction::Left,
                '>' => Direction::Right,
                'v' => Direction::Down,
                _ => panic!("Wrong input"),
            });
        }
    }
    (map_output, max_map_len, commands_output)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "2028");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "");
    }
}
