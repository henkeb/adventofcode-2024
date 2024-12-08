use std::collections::{HashMap, HashSet};

pub fn puzzle_1(input: &str) -> String {
    let (map, len) = handle_input(input);
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for (_, points) in map.iter() {
        for (i, point) in points.iter().enumerate() {
            for j in (i + 1)..points.len() {
                for node in calculate_antinode_placement(point, &points[j], &len, false) {
                    antinodes.insert(node);
                }
            }
        }
    }

    antinodes.len().to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let (map, len) = handle_input(input);
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for (_, points) in map.iter() {
        for (i, point) in points.iter().enumerate() {
            for j in (i + 1)..points.len() {
                for node in calculate_antinode_placement(point, &points[j], &len, true) {
                    antinodes.insert(node);
                }
            }
        }
    }

    antinodes.len().to_string()
}

fn calculate_antinode_placement(
    p_1: &(isize, isize),
    p_2: &(isize, isize),
    len: &(isize, isize),
    scalar: bool,
) -> Vec<(isize, isize)> {
    let check_bounds = |a: (isize, isize), b: (isize, isize)| -> bool {
        if a.0 >= 0 && a.0 < b.0 {
            if a.1 >= 0 && a.1 < b.1 {
                return true;
            }
        }
        false
    };
    let mut output = Vec::new();

    let (dx, dy) = (p_2.0 - p_1.0, p_2.1 - p_1.1);

    let mut antinode = (p_1.0 - dx, p_1.1 - dy);

    if scalar {
        output.push(*p_1); // Get the node we start from!
        while check_bounds(antinode, *len) {
            output.push(antinode);
            antinode.0 -= dx;
            antinode.1 -= dy;
        }
        antinode = (p_1.0 + dx, p_1.1 + dy);
        while check_bounds(antinode, *len) {
            output.push(antinode);
            antinode.0 += dx;
            antinode.1 += dy;
        }
    } else {
        if check_bounds(antinode, *len) {
            output.push(antinode);
        }
        antinode = (p_2.0 + dx, p_2.1 + dy);
        if check_bounds(antinode, *len) {
            output.push(antinode);
        }
    }

    output
}

fn handle_input(input: &str) -> (HashMap<char, Vec<(isize, isize)>>, (isize, isize)) {
    let mut map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    let num_rows = input.lines().count() as isize;
    let num_columns = input.lines().next().unwrap().len() as isize;

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                map.entry(ch)
                    .and_modify(|point_vec| point_vec.push((x as isize, y as isize)))
                    .or_insert(vec![(x as isize, y as isize)]);
            }
        }
    }
    (map, (num_columns, num_rows))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "14");
    }

    #[test]
    fn test_2a() {
        let input = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        assert_eq!(puzzle_2(&input), "9");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "34");
    }
}
