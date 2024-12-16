use std::collections::{HashMap, HashSet, VecDeque};

pub fn puzzle_1(input: &str) -> String {
    let mut map = handle_input(input);
    let starting_position = find_starting_position(&map);
    bfs_with_cost(&mut map, starting_position).0.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mut map = handle_input(input);
    let starting_position = find_starting_position(&map);
    bfs_with_cost(&mut map, starting_position).1.to_string()
}

fn bfs_with_cost(map: &mut Vec<Vec<char>>, starting_position: (isize, isize)) -> (usize, usize) {
    let mut queue: VecDeque<((isize, isize), Direction, usize, Vec<(isize, isize)>)> =
        VecDeque::new();
    queue.push_back((
        starting_position,
        Direction::East,
        0,
        vec![starting_position],
    ));
    let mut output = usize::max_value();
    let mut visited: HashMap<(isize, isize), usize> = HashMap::new();
    let mut best_path: HashSet<(isize, isize)> = HashSet::new();
    while let Some((position, direction, cost, path)) = queue.pop_front() {
        if map[position.1 as usize][position.0 as usize] == '#' {
            continue;
        } else if map[position.1 as usize][position.0 as usize] == 'E' {
            if cost < output {
                output = cost;
                best_path.clear();
                for tile in path {
                    best_path.insert(tile);
                }
            } else if cost == output {
                for tile in path {
                    best_path.insert(tile);
                }
            }
            continue;
        }
        if let Some(prev_cost) = visited.get(&position) {
            if cost > *prev_cost + 1000 {
                continue;
            }
        }
        visited
            .entry(position)
            .and_modify(|old_cost| *old_cost = cost)
            .or_insert(cost);
        let mut path_same_dir = path.clone();
        path_same_dir.push((
            position.0 + direction.value().0,
            position.1 + direction.value().1,
        ));
        queue.push_back((
            (
                position.0 + direction.value().0,
                position.1 + direction.value().1,
            ),
            direction.clone(),
            cost + 1,
            path_same_dir,
        ));
        let (rotation_1, rotation_2) = direction.rotations();
        let mut path_rotation_1 = path.clone();
        path_rotation_1.push((position.0 + rotation_1.0 .0, position.1 + rotation_1.0 .1));
        queue.push_back((
            (position.0 + rotation_1.0 .0, position.1 + rotation_1.0 .1),
            rotation_1.1,
            cost + 1000 + 1,
            path_rotation_1,
        ));
        let mut path_rotation_2 = path.clone();
        path_rotation_2.push((position.0 + rotation_2.0 .0, position.1 + rotation_2.0 .1));
        queue.push_back((
            (position.0 + rotation_2.0 .0, position.1 + rotation_2.0 .1),
            rotation_2.1,
            cost + 1000 + 1,
            path_rotation_2,
        ));
    }
    for tile in best_path.iter() {
        map[tile.1 as usize][tile.0 as usize] = 'O';
    }
    (output, best_path.len())
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotations(&self) -> (((isize, isize), Self), ((isize, isize), Self)) {
        match self {
            Direction::North | Direction::South => {
                (((1, 0), Direction::East), ((-1, 0), Direction::West))
            }
            Direction::East | Direction::West => {
                (((0, 1), Direction::South), ((0, -1), Direction::North))
            }
        }
    }
    fn value(&self) -> (isize, isize) {
        match self {
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
            Direction::North => (0, -1),
            Direction::South => (0, 1),
        }
    }
}

fn handle_input(input: &str) -> Vec<Vec<char>> {
    let mut output = Vec::new();
    for (y, row) in input.lines().enumerate() {
        output.push(vec![]);
        for ch in row.chars() {
            output[y].push(ch);
        }
    }
    output
}

fn find_starting_position(map: &Vec<Vec<char>>) -> (isize, isize) {
    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == 'S' {
                return (x as isize, y as isize);
            }
        }
    }
    panic!("No starting position found");
}

fn display_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        for ch in row.iter() {
            print!("{ch}");
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    const INPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_1a() {
        assert_eq!(puzzle_1(&INPUT), "7036");
    }
    #[test]
    fn test_1b() {
        assert_eq!(puzzle_1(&INPUT2), "11048");
    }

    #[test]
    fn test_2a() {
        assert_eq!(puzzle_2(&INPUT), "45");
    }
    #[test]
    fn test_2b() {
        assert_eq!(puzzle_2(&INPUT2), "64");
    }
}
