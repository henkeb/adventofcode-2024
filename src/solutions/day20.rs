use std::collections::{HashMap, HashSet};

pub fn puzzle_1(input: &str) -> String {
    let map = handle_input(input);
    let start_pos = find_start_pos(&map);
    let end_pos = find_end_pos(&map);
    let mut tiles = dfs(&map, start_pos, end_pos);
    update_tiles(&mut tiles, end_pos);

    dbg!(check_shortcuts(&map, &tiles, start_pos, end_pos, 2));
    check_shortcuts(&map, &tiles, start_pos, end_pos, 2)
        .iter()
        .filter(|(&cost, _)| cost >= 100)
        .map(|(_, num)| num)
        .sum::<usize>()
        .to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let map = handle_input(input);
    let start_pos = find_start_pos(&map);
    let end_pos = find_end_pos(&map);
    let mut tiles = dfs(&map, start_pos, end_pos);
    update_tiles(&mut tiles, end_pos);
    check_shortcuts(&map, &tiles, start_pos, end_pos, 20)
        .iter()
        .filter(|(&saved_cost, _)| saved_cost >= 100)
        .map(|(_, num)| num)
        .sum::<usize>()
        .to_string()
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
    cheat_distance: isize,
) -> HashMap<usize, usize> {
    let max_len: (isize, isize) = (map[0].len() as isize, map.len() as isize);
    let mut active_point = start_pos.clone();
    let mut cheat_paths: HashMap<usize, usize> = HashMap::new();
    while active_point != end_pos {
        for dy in -cheat_distance..=cheat_distance {
            let new_y = active_point.1 + dy;

            let remaining = cheat_distance - dy.abs();
            for dx in -remaining..=remaining {
                let new_x = active_point.0 + dx;
                if !is_tile_valid((new_x, new_y), max_len) {
                    continue;
                }

                if !is_tile_free(map, &(new_x, new_y)) {
                    continue;
                }
                let cost = tiles[new_y as usize][new_x as usize].cost;
                let cheated_distance = dy.abs() + dx.abs();
                let total_saved_cost = cost as isize
                    - tiles[active_point.1 as usize][active_point.0 as usize].cost as isize
                    - cheated_distance;
                if total_saved_cost.is_negative() {
                    continue;
                }
                cheat_paths
                    .entry(total_saved_cost as usize)
                    .and_modify(|cnt| *cnt += 1)
                    .or_insert(1);
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

fn manhattan_distance(active_position: (isize, isize), end_pos: (isize, isize)) -> isize {
    (active_position.0.abs_diff(end_pos.0) + active_position.1.abs_diff(end_pos.1)) as isize
}

fn is_tile_free(map: &Vec<Vec<char>>, point: &(isize, isize)) -> bool {
    map[point.1 as usize][point.0 as usize] != '#'
}

fn update_tiles(tile_details: &mut Vec<Vec<Tile>>, end: (isize, isize)) {
    let mut active_point = end.clone();
    let mut next = end;
    while active_point != tile_details[active_point.1 as usize][active_point.0 as usize].previous {
        active_point = tile_details[active_point.1 as usize][active_point.0 as usize]
            .previous
            .clone();
        tile_details[active_point.1 as usize][active_point.0 as usize].next = next;
        next = active_point.clone();
    }
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

    // #[test]
    // fn test_1() {
    //     assert_eq!(puzzle_1(&INPUT), "");
    // }

    // #[test]
    // fn test_2() {
    //     assert_eq!(puzzle_2(&INPUT), "");
    // }
}
