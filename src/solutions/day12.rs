use std::collections::HashSet;

pub fn puzzle_1(input: &str) -> String {
    let map = handle_input(input);
    count_fence_cost(&map, false).to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let map = handle_input(input);
    count_fence_cost(&map, true).to_string()
}

fn count_fence_cost(map: &Vec<Vec<char>>, has_discount: bool) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let max_len = (map[0].len() as isize, map.len() as isize);
    let mut total_cost = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if visited.contains(&(x, y)) {
                continue;
            }
            let mut queue: Vec<(usize, usize)> = Vec::new();
            let plant_type = val;
            queue.push((x, y));
            let mut this_plant = Vec::new();
            while let Some(square) = queue.pop() {
                if visited.contains(&square) {
                    continue;
                }
                if map[square.1][square.0] != *plant_type {
                    continue;
                }
                visited.insert(square);
                this_plant.push((square, plant_type));
                for dir in DIRECTION.iter() {
                    if check_bounds(&square, *dir, max_len) {
                        queue.push((
                            (square.0 as isize + dir.0) as usize,
                            (square.1 as isize + dir.1) as usize,
                        ));
                    }
                }
            }
            if has_discount {
                total_cost += calculate_fence_cost_with_discount(map, &this_plant, max_len);
            } else {
                total_cost += calculate_fence_cost(map, &this_plant, max_len);
            }
        }
    }
    total_cost
}

fn calculate_fence_cost_with_discount(
    map: &Vec<Vec<char>>,
    points: &Vec<((usize, usize), &char)>,
    max_len: (isize, isize),
) -> usize {
    let mut total_sides = 0;
    let plants_hset: HashSet<(usize, usize)> =
        HashSet::from_iter(points.iter().map(|plants| plants.0));

    // check in y_direction
    // let mut check_y = HashSet::new();
    let up_down: [(isize, isize); 2] = [(0, 1), (0, -1)];
    let left_right: [(isize, isize); 2] = [(-1, 0), (1, 0)];
    // left side
    let mut visited = HashSet::new();
    let mut visited_updown = HashSet::new();
    let mut left_set = HashSet::new();
    let mut right_set = HashSet::new();
    let mut up_set = HashSet::new();
    let mut down_set = HashSet::new();
    let (mut left_count, mut right_count, mut up_count, mut down_count) = (0, 0, 0, 0);
    for (point, &plant) in points.iter() {
        left_set.clear();
        right_set.clear();
        let mut queue: Vec<(usize, usize)> = Vec::new();
        queue.push(*point);
        while let Some(point) = queue.pop() {
            if visited.contains(&point) {
                println!("visited point: {point:?}");
                continue;
            }
            visited.insert(point);
            if check_bounds(&point, (-1, 0), max_len) {
                let new_point = (
                    (point.0 as isize + -1) as usize,
                    (point.1 as isize + 0) as usize,
                );
                if !plants_hset.contains(&new_point) {
                    left_set.insert(point);
                }
            } else {
                left_set.insert(point);
            }
            if check_bounds(&point, (1, 0), max_len) {
                let new_point = (
                    (point.0 as isize + 1) as usize,
                    (point.1 as isize + 0) as usize,
                );
                if !plants_hset.contains(&new_point) {
                    right_set.insert(point);
                }
            } else {
                right_set.insert(point);
            }
            for dir in up_down.iter() {
                if check_bounds(&point, *dir, max_len) {
                    let new_point = (
                        (point.0 as isize + dir.0) as usize,
                        (point.1 as isize + dir.1) as usize,
                    );
                    if map[new_point.1][new_point.0] == plant {
                        queue.push(new_point.clone());
                    }
                }
            }
        }
        if !left_set.is_empty() {
            left_count += 1;
            total_sides += 1;
        }
        if !right_set.is_empty() {
            println!("Right count incremented by pos {point:?}, plant {plant}");
            right_count += 1;
            total_sides += 1;
        }
        queue.clear();
        up_set.clear();
        down_set.clear();
        queue.push(*point);
        while let Some(point) = queue.pop() {
            if visited_updown.contains(&point) {
                continue;
            }
            visited_updown.insert(point);
            if check_bounds(&point, (0, -1), max_len) {
                let new_point = (
                    (point.0 as isize + 0) as usize,
                    (point.1 as isize + -1) as usize,
                );
                if !plants_hset.contains(&new_point) {
                    up_set.insert(point);
                }
            } else {
                up_set.insert(point);
            }
            if check_bounds(&point, (0, 1), max_len) {
                let new_point = (
                    (point.0 as isize + 0) as usize,
                    (point.1 as isize + 1) as usize,
                );
                if !plants_hset.contains(&new_point) {
                    down_set.insert(point);
                }
            } else {
                down_set.insert(point);
            }
            for dir in left_right.iter() {
                if check_bounds(&point, *dir, max_len) {
                    let new_point = (
                        (point.0 as isize + dir.0) as usize,
                        (point.1 as isize + dir.1) as usize,
                    );
                    if map[new_point.1][new_point.0] == plant {
                        queue.push(new_point.clone());
                    }
                }
            }
        }
        // println!("down set: {down_set:?}");
        // println!("up set: {up_set:?}");
        if !down_set.is_empty() {
            down_count += 1;
            total_sides += 1;
        }
        if !up_set.is_empty() {
            up_count += 1;
            total_sides += 1;
        }
        // println!("total_sides {total_sides} for {}", plant);
    }
    println!("left {left_count}, right {right_count}, up {up_count}, down {down_count}");
    total_sides * points.len()
}

fn calculate_fence_cost(
    map: &Vec<Vec<char>>,
    points: &Vec<((usize, usize), &char)>,
    max_len: (isize, isize),
) -> usize {
    let mut fence_length = 0;
    for (point, &plant_type) in points.iter() {
        for dir in DIRECTION.iter() {
            if check_bounds(&point, *dir, max_len) {
                let new_point = (
                    (point.0 as isize + dir.0) as usize,
                    (point.1 as isize + dir.1) as usize,
                );
                if map[new_point.1][new_point.0] == plant_type {
                    continue;
                }
            }
            fence_length += 1;
        }
    }
    fence_length * points.len()
}

fn check_bounds(new_point: &(usize, usize), b: (isize, isize), max_len: (isize, isize)) -> bool {
    let new_point = (new_point.0 as isize + b.0, new_point.1 as isize + b.1);
    if new_point.0 >= 0 && new_point.0 < max_len.0 {
        if new_point.1 >= 0 && new_point.1 < max_len.1 {
            return true;
        }
    }
    false
}

fn handle_input(input: &str) -> Vec<Vec<char>> {
    let mut output = Vec::new();
    for row in input.lines() {
        let mut line = Vec::new();
        for ch in row.chars() {
            line.push(ch);
        }
        output.push(line);
    }
    output
}

const DIRECTION: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "AAAA
BBCD
BBCC
EEEC";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "140");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "80");
    }
    const INPUT2: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    #[test]
    fn test_3() {
        assert_eq!(puzzle_2(&INPUT2), "236");
    }
}
