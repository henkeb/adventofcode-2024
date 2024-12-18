use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    ops::Add,
    vec,
};

use regex::Regex;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone)]
struct Tile {
    parent: Point,
    f: isize,
    g: isize,
    h: isize,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd)]
struct HeapElement {
    f: isize,
    point: Point,
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f.cmp(&other.f)
    }
}

impl HeapElement {
    fn new(f: isize, point: Point) -> Self {
        HeapElement { f, point }
    }
}

impl Tile {
    fn new() -> Self {
        Tile {
            parent: Point { x: -1, y: -1 },
            f: isize::max_value(),
            g: isize::max_value(),
            h: isize::max_value(),
        }
    }
}

pub fn puzzle_1(input: &str) -> String {
    let bytes = handle_input(input);
    let mut map = vec![vec!['.'; 7]; 7];
    for (i, byte) in bytes.into_iter().enumerate() {
        map[byte.1 as usize][byte.0 as usize] = '#';
        if i == 11 {
            break;
        }
    }
    a_star(&map, Point { x: 0, y: 0 }, Point { x: 6, y: 6 })
        .len()
        .to_string()

    // let mut map = vec![vec!['.'; 71]; 71];
    // for (i, byte) in bytes.into_iter().enumerate() {
    //     map[byte.1 as usize][byte.0 as usize] = '#';
    //     if i == 1023 {
    //         break;
    //     }
    // }
    //
    // a_star(&map, Point { x: 0, y: 0 }, Point { x: 70, y: 70 })
    //     .len()
    //     .to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let bytes = handle_input(input);
    let mut map = vec![vec!['.'; 7]; 7];
    for (i, byte) in bytes.iter().enumerate() {
        map[byte.1 as usize][byte.0 as usize] = '#';
        if i < 12 {
            continue;
        }
        if a_star(&map, Point { x: 0, y: 0 }, Point { x: 6, y: 6 }).is_empty() {
            return format!("{},{}", byte.0, byte.1);
        }
    }

    // let mut map = vec![vec!['.'; 71]; 71];
    // for (i, byte) in bytes.iter().enumerate() {
    //     map[byte.1 as usize][byte.0 as usize] = '#';
    //     if i < 1024 {
    //         continue;
    //     }
    //     if a_star(&map, Point { x: 0, y: 0 }, Point { x: 70, y: 70 }).is_empty() {
    //         return format!("{},{}", byte.0, byte.1);
    //     }
    // }
    "Not found!".to_string()
}

// Utility functions
fn is_tile_valid(point: &Point, max_len: &Point) -> bool {
    if point.x >= 0 && point.x < max_len.x {
        if point.y >= 0 && point.y < max_len.y {
            return true;
        }
    }
    false
}

fn is_tile_blocked(map: &Vec<Vec<char>>, point: &Point) -> bool {
    if map[point.y as usize][point.x as usize] == '.' {
        true
    } else {
        false
    }
}

fn calculate_h_value(current: &Point, target: &Point) -> isize {
    (target.x - current.x).abs() + (target.y - current.y).abs()
}

fn get_path(tile_details: &Vec<Vec<Tile>>, end: &Point) -> HashSet<Point> {
    let mut active_point = end.clone();
    let mut path = HashSet::new();
    while active_point != tile_details[active_point.y as usize][active_point.x as usize].parent {
        path.insert(active_point.clone());
        active_point = tile_details[active_point.y as usize][active_point.x as usize]
            .parent
            .clone();
    }
    path
}

fn a_star(map: &Vec<Vec<char>>, start: Point, end: Point) -> HashSet<Point> {
    let max_len = Point {
        x: end.x + 1,
        y: end.y + 1,
    };

    let mut closed_list: HashSet<Point> = HashSet::new();
    let mut tile_details: Vec<Vec<Tile>> =
        vec![vec![Tile::new(); max_len.x as usize]; max_len.y as usize];

    let mut open_list: BinaryHeap<Reverse<HeapElement>> = BinaryHeap::new();

    // Insert starting cell
    open_list.push(Reverse(HeapElement::new(0, start.clone())));
    let mut new_tile = Tile::new();
    new_tile.f = 0;
    new_tile.g = 0;
    new_tile.h = 0;
    new_tile.parent = start.clone();

    tile_details[start.y as usize][start.x as usize] = new_tile;

    while let Some(Reverse(node)) = open_list.pop() {
        closed_list.insert(node.point.clone());
        // Check North
        let north = Point {
            x: node.point.x,
            y: node.point.y - 1,
        };
        if do_checks(
            &node.point,
            north,
            &max_len,
            &end,
            &mut tile_details,
            &mut closed_list,
            &mut open_list,
            map,
        ) {
            return get_path(&tile_details, &end);
        }
        // Check South
        let south = Point {
            x: node.point.x,
            y: node.point.y + 1,
        };
        if do_checks(
            &node.point,
            south,
            &max_len,
            &end,
            &mut tile_details,
            &mut closed_list,
            &mut open_list,
            map,
        ) {
            return get_path(&tile_details, &end);
        }
        // Check West
        let west = Point {
            x: node.point.x - 1,
            y: node.point.y,
        };
        if do_checks(
            &node.point,
            west,
            &max_len,
            &end,
            &mut tile_details,
            &mut closed_list,
            &mut open_list,
            map,
        ) {
            return get_path(&tile_details, &end);
        }
        // Check East
        let east = Point {
            x: node.point.x + 1,
            y: node.point.y,
        };
        if do_checks(
            &node.point,
            east,
            &max_len,
            &end,
            &mut tile_details,
            &mut closed_list,
            &mut open_list,
            map,
        ) {
            return get_path(&tile_details, &end);
        }
    }
    HashSet::default()
}

fn do_checks(
    parent: &Point,
    new_point: Point,
    max_len: &Point,
    end: &Point,
    tile_details: &mut Vec<Vec<Tile>>,
    closed_list: &mut HashSet<Point>,
    open_list: &mut BinaryHeap<Reverse<HeapElement>>,
    map: &Vec<Vec<char>>,
) -> bool {
    if is_tile_valid(&new_point, &max_len) {
        if new_point == *end {
            tile_details[new_point.y as usize][new_point.x as usize].parent = parent.clone();
            return true;
        } else if !closed_list.contains(&new_point)
            && map[new_point.y as usize][new_point.x as usize] == '.'
        {
            let g_new = tile_details[parent.y as usize][parent.x as usize].g + 1;
            let h_new = calculate_h_value(&new_point, &end);
            let f_new = g_new + h_new;

            if tile_details[new_point.y as usize][new_point.x as usize].f == isize::max_value()
                || tile_details[new_point.y as usize][new_point.x as usize].f > f_new
            {
                tile_details[new_point.y as usize][new_point.x as usize].f = f_new;
                tile_details[new_point.y as usize][new_point.x as usize].g = g_new;
                tile_details[new_point.y as usize][new_point.x as usize].h = h_new;
                tile_details[new_point.y as usize][new_point.x as usize].parent = parent.clone();
                open_list.push(Reverse(HeapElement::new(f_new, new_point)));
            }
        }
    }
    false
}

fn handle_input(input: &str) -> Vec<(isize, isize)> {
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut results = vec![];
    for line in input.lines() {
        for (_, [x, y]) in re.captures_iter(line).map(|c| c.extract()) {
            results.push((x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()));
        }
    }
    results
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

    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "22");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "6,1");
    }
}
