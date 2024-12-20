use std::collections::{HashMap, HashSet};

pub fn puzzle_1(input: &str) -> String {
    let map = handle_input(input);
    let start_pos = find_start_pos(&map);
    let end_pos = find_end_pos(&map);
    let mut tiles = dfs(&map, start_pos, end_pos);
    let _path = get_path(&mut tiles, end_pos);

    check_shortcuts(&map, &tiles, start_pos, end_pos)
        .iter()
        .filter(|(_, &cost)| cost >= 100)
        .count()
        .to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let _ = handle_input(input);
    "Not implemented yet!".to_string()
}

#[derive(Clone, Debug)]
struct Tile {
    previous: (isize, isize),
    next: (isize, isize),
    cost: usize,
}

impl Tile {
    fn new() -> Self {
        Tile {
            previous: (-1, -1),
            next: (-1, -1),
            cost: usize::MAX,
        }
    }
}
fn check_shortcuts(
    map: &Vec<Vec<char>>,
    tiles: &Vec<Vec<Tile>>,
    start_pos: (isize, isize),
    end_pos: (isize, isize),
) -> HashMap<(isize, isize), usize> {
    let max_len: (isize, isize) = (map[0].len() as isize, map.len() as isize);
    let mut active_point = start_pos.clone();
    let mut cheat_paths: HashMap<(isize, isize), usize> = HashMap::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    while active_point != end_pos {
        for (x, y) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            if !is_tile_valid((active_point.0 + x, active_point.1 + y), max_len)
                || is_tile_free(&map, &(active_point.0 + x, active_point.1 + y))
            {
                continue;
            }
            if is_tile_valid((active_point.0 + x * 2, active_point.1 + y * 2), max_len)
                && is_tile_free(&map, &(active_point.0 + x * 2, active_point.1 + y * 2))
            {
                if visited.contains(&(active_point.0 + x * 2, active_point.1 + y * 2)) {
                    continue;
                }
                // let jump = active_point;
                // let jump_to = (
                //     active_point.0 + x * 2 as isize,
                //     active_point.1 + y * 2 as isize,
                // );
                // let cost_to_jump = tiles[active_point.1 as usize][active_point.0 as usize].cost;
                // let cost_after_jump = tiles[(active_point.1 + y * 2) as usize]
                //     [(active_point.0 + x * 2) as usize]
                //     .cost;
                // let cost_jumper = total_cost
                //     - tiles[(active_point.1 + y * 2) as usize][(active_point.0 + x * 2) as usize]
                //         .cost;
                // println!("Before jump: {jump:?}, cost to this point {cost_to_jump}");
                // println!("After jumper: {jump_to:?}, cost to this point {cost_after_jump}");
                // println!(
                //     "After jump: {jump_to:?}, cost to this point {}",
                //     total_cost - cost_to_jump
                // );
                //
                cheat_paths.insert(
                    (active_point.0 + x, active_point.1 + y),
                    tiles[(active_point.1 + y * 2) as usize][(active_point.0 + x * 2) as usize]
                        .cost
                        - tiles[active_point.1 as usize][active_point.0 as usize].cost
                        - 2,
                );
                visited.insert(active_point);
            }
        }
        active_point = tiles[active_point.1 as usize][active_point.0 as usize].next
    }
    cheat_paths
}
fn dfs(map: &Vec<Vec<char>>, start_pos: (isize, isize), end_pos: (isize, isize)) -> Vec<Vec<Tile>> {
    let max_len: (isize, isize) = (map[0].len() as isize, map.len() as isize);
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut queue: Vec<((isize, isize), (isize, isize), usize)> = Vec::new();
    let mut tiles: Vec<Vec<Tile>> = vec![vec![Tile::new(); max_len.0 as usize]; max_len.1 as usize];

    queue.push((start_pos, start_pos, 0));

    while let Some((new_position, previous_position, cost)) = queue.pop() {
        if visited.contains(&new_position) {
            continue;
        }

        visited.insert(new_position);
        tiles[new_position.1 as usize][new_position.0 as usize].previous = previous_position;
        tiles[new_position.1 as usize][new_position.0 as usize].cost = cost;

        if new_position == end_pos {
            return tiles;
        }

        for (x, y) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            if !is_tile_valid((new_position.0 + x, new_position.1 + y), max_len)
                || !is_tile_free(&map, &(new_position.0 + x, new_position.1 + y))
            {
                continue;
            }
            queue.push((
                (new_position.0 + x, new_position.1 + y),
                new_position,
                cost + 1,
            ));
        }
    }
    panic!("Couldnt find end node");
}

fn is_tile_free(map: &Vec<Vec<char>>, point: &(isize, isize)) -> bool {
    map[point.1 as usize][point.0 as usize] != '#'
}

fn get_path(tile_details: &mut Vec<Vec<Tile>>, end: (isize, isize)) -> HashSet<(isize, isize)> {
    let mut active_point = end.clone();
    let mut next = end;
    let mut path = HashSet::new();
    while active_point != tile_details[active_point.1 as usize][active_point.0 as usize].previous {
        path.insert(active_point.clone());
        active_point = tile_details[active_point.1 as usize][active_point.0 as usize]
            .previous
            .clone();
        tile_details[active_point.1 as usize][active_point.0 as usize].next = next;
        next = active_point.clone();
    }
    path
}

fn is_tile_valid(point: (isize, isize), max_len: (isize, isize)) -> bool {
    if point.0 >= 0 && point.0 < max_len.0 {
        if point.1 >= 0 && point.1 < max_len.1 {
            return true;
        }
    }
    false
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

fn find_end_pos(map: &Vec<Vec<char>>) -> (isize, isize) {
    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == 'E' {
                return (x as isize, y as isize);
            }
        }
    }
    panic!("Cant find end pos");
}

fn find_start_pos(map: &Vec<Vec<char>>) -> (isize, isize) {
    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == 'S' {
                return (x as isize, y as isize);
            }
        }
    }
    panic!("Cant find start pos");
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
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "");
    }
}
