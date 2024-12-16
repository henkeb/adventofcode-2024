use std::collections::{HashSet, VecDeque};

pub fn puzzle_1(input: &str) -> String {
    let (mut map, max_map_len, commands) = handle_input(input, false);
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
    let (mut map, max_map_len, commands) = handle_input(input, true);

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
            '[' | ']' => {
                if *command == Direction::Left || *command == Direction::Right {
                    if !move_boxes_if_possible2_left_right(&mut map, command, &new_position) {
                        continue;
                    }
                } else {
                    if !move_boxes_if_possible_up_down(&mut map, command, &new_position) {
                        continue;
                    }
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
    calculate_gps_coordinates2(&map).to_string()
}

fn display_map(robot_map: &Vec<Vec<char>>) {
    for row in robot_map.iter() {
        for ch in row.iter() {
            print!("{ch}");
        }
        print!("\n");
    }
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

fn calculate_gps_coordinates2(map: &Vec<Vec<char>>) -> usize {
    let mut output = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == '[' {
                output += y * 100 + x;
            }
        }
    }
    output
}
fn move_boxes_if_possible2_left_right(
    map: &mut Vec<Vec<char>>,
    command: &Direction,
    new_position: &(isize, isize),
) -> bool {
    let mut find_empty_position = new_position.clone();

    while map[find_empty_position.1 as usize][find_empty_position.0 as usize] != '#' {
        find_empty_position.0 = find_empty_position.0 + command.value().0;

        if map[find_empty_position.1 as usize][find_empty_position.0 as usize] == '.' {
            if *command == Direction::Left {
                for x in find_empty_position.0..new_position.0 {
                    map[find_empty_position.1 as usize][x as usize] =
                        map[find_empty_position.1 as usize][(x + 1) as usize];
                }
            } else {
                for x in ((new_position.0 + 1)..=find_empty_position.0).rev() {
                    map[find_empty_position.1 as usize][x as usize] =
                        map[find_empty_position.1 as usize][(x - 1) as usize];
                }
            }
            map[new_position.1 as usize][new_position.0 as usize] = '.';
            return true;
        }
    }
    false
}

fn move_boxes_if_possible_up_down(
    map: &mut Vec<Vec<char>>,
    command: &Direction,
    robot_position: &(isize, isize),
) -> bool {
    let new_position = (robot_position.0, robot_position.1);
    // (Left box side), (Right box side)
    let mut boxes_to_be_moved: HashSet<((isize, isize), (isize, isize))> = HashSet::new();
    let mut queue: VecDeque<((isize, isize), (isize, isize))> = VecDeque::new();
    let mut new_map = map.clone();
    let mut _first_move = ((0, 0), (0, 0));
    if map[new_position.1 as usize][new_position.0 as usize] == ']' {
        queue.push_back(((new_position.0 - 1, new_position.1), (new_position)));
        _first_move = ((new_position.0 - 1, new_position.1), (new_position));
    } else {
        queue.push_back(((new_position), (new_position.0 + 1, new_position.1)));
        _first_move = ((new_position), (new_position.0 + 1, new_position.1));
    }
    while let Some((left, right)) = queue.pop_front() {
        if boxes_to_be_moved.contains(&(left, right)) {
            continue;
        }
        if map[left.1 as usize][left.0 as usize] == '['
            && map[right.1 as usize][right.0 as usize] == ']'
        {
            boxes_to_be_moved.insert((left, right));
        }
        let left_new = ((left.0 + command.value().0), (left.1 + command.value().1));
        match map[left_new.1 as usize][left_new.0 as usize] {
            '#' => return false,
            ']' => {
                queue.push_back(((left_new.0 - 1, left_new.1), (left_new)));
            }
            '[' => {
                queue.push_back((left_new, (left_new.0 + 1, left_new.1)));
            }
            '.' => (),
            _ => panic!("Wrong char"),
        }

        let right_new = ((right.0 + command.value().0), (right.1 + command.value().1));
        match map[right_new.1 as usize][right.0 as usize] {
            '#' => return false,
            ']' => {
                queue.push_back(((right_new.0 - 1, right_new.1), (right_new)));
            }
            '[' => {
                queue.push_back((right_new, (right_new.0 + 1, right_new.1)));
            }
            '.' => (),
            _ => panic!("Wrong char"),
        }
    }
    let mut values_sorted: Vec<((isize, isize), (isize, isize))> = Vec::new();
    for val in boxes_to_be_moved.iter() {
        values_sorted.push(*val);
    }
    if *command == Direction::Up {
        values_sorted.sort_by(|a, b| a.1 .1.cmp(&b.1 .1));
    } else {
        values_sorted.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));
    }
    for (box_left, box_right) in values_sorted.iter() {
        let left_new = (
            (box_left.0 + command.value().0),
            (box_left.1 + command.value().1),
        );
        new_map[left_new.1 as usize][left_new.0 as usize] = '[';
        new_map[box_left.1 as usize][box_left.0 as usize] = '.';
        let right_new = (
            (box_right.0 + command.value().0),
            (box_right.1 + command.value().1),
        );
        new_map[right_new.1 as usize][right_new.0 as usize] = ']';
        new_map[box_right.1 as usize][box_right.0 as usize] = '.';
    }
    *map = new_map;
    map[new_position.1 as usize][new_position.0 as usize] = '@';
    map[robot_position.1 as usize][robot_position.0 as usize] = '.';
    true
}

fn verify_line_no_wall_in_line(map: &Vec<Vec<char>>, range: &(usize, usize), row: &usize) -> bool {
    for ch in &map[*row][range.0..=range.1] {
        if *ch == '#' {
            return false;
        }
    }
    true
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

#[derive(Debug, Eq, PartialEq)]
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
fn handle_input(
    input: &str,
    twice_as_wide: bool,
) -> (Vec<Vec<char>>, (isize, isize), Vec<Direction>) {
    let (map, commands) = input.split_once("\n\n").unwrap();
    let mut map_output: Vec<Vec<char>> = Vec::new();
    for (i, line) in map.lines().enumerate() {
        map_output.push(vec![]);
        for ch in line.chars() {
            if twice_as_wide {
                match ch {
                    '#' => {
                        map_output[i].push('#');
                        map_output[i].push('#');
                    }
                    'O' => {
                        map_output[i].push('[');
                        map_output[i].push(']');
                    }
                    '@' => {
                        map_output[i].push('@');
                        map_output[i].push('.');
                    }
                    '.' => {
                        map_output[i].push('.');
                        map_output[i].push('.');
                    }
                    _ => panic!("Wrong char"),
                }
            } else {
                map_output[i].push(ch);
            }
        }
    }
    let max_map_len = (map_output[0].len() as isize, map_output.len() as isize);

    let mut commands_output: Vec<Direction> = Vec::with_capacity(commands.chars().count());

    for line in commands.lines() {
        for ch in line.trim().chars() {
            commands_output.push(match ch {
                '^' => Direction::Up,
                '<' => Direction::Left,
                '>' => Direction::Right,
                'v' => Direction::Down,
                _ => {
                    dbg!(ch);
                    panic!("Wrong input")
                }
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
    const INPUT2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_1a() {
        assert_eq!(puzzle_1(&INPUT), "2028");
    }
    #[test]
    fn test_1b() {
        assert_eq!(puzzle_1(&INPUT2), "10092");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT2), "9021");
    }
}
