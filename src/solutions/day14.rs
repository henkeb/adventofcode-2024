use regex::Regex;
pub fn puzzle_1(input: &str) -> String {
    let max_len_test = (101, 103);
    // let max_len_test = (11, 7);
    let mut robots = handle_input(input, 100, max_len_test);

    let mut quadrant_counter: [usize; 4] = [0; 4];
    dbg!(&robots);

    for robot in robots.iter_mut() {
        robot.position.0 = robot.position.0 % max_len_test.0;
        robot.position.1 = robot.position.1 % max_len_test.1;
        if robot.position.0.is_negative() {
            robot.position.0 += max_len_test.0;
        }
        if robot.position.1.is_negative() {
            robot.position.1 += max_len_test.1;
        }
        if robot.position.0 < max_len_test.0 / 2 {
            if robot.position.1 < max_len_test.1 / 2 {
                quadrant_counter[0] += 1;
            } else if robot.position.1 < max_len_test.1 && robot.position.1 > max_len_test.1 / 2 {
                quadrant_counter[1] += 1;
            }
        } else if robot.position.0 > max_len_test.0 / 2 && robot.position.0 < max_len_test.0 {
            if robot.position.1 < max_len_test.1 / 2 {
                quadrant_counter[2] += 1;
            } else if robot.position.1 < max_len_test.1 && robot.position.1 > max_len_test.1 / 2 {
                quadrant_counter[3] += 1;
            }
        }
    }
    quadrant_counter.iter().product::<usize>().to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let max_len: (isize, isize) = (101, 103);
    let mut robots = handle_input(input, 1, max_len);
    let mut robot_map: Vec<Vec<char>> = vec![vec!['.'; max_len.0 as usize]; max_len.1 as usize];
    for i in 0..=100 {
        println!("NEW TREE HERE loop {i}: \n\n",);
        display_tree(&robot_map);
        println!("\n\n");
        for robot in robots.iter_mut() {
            robot_map[robot.position.1 as usize][robot.position.0 as usize] = '.';
            robot.position.0 += robot.velocity.0;
            robot.position.1 += robot.velocity.1;

            robot.position.0 = robot.position.0 % max_len.0;
            robot.position.1 = robot.position.1 % max_len.1;
            if robot.position.0.is_negative() {
                robot.position.0 += max_len.0;
            }
            if robot.position.1.is_negative() {
                robot.position.1 += max_len.1;
            }
            if robot.position.0 >= max_len.0 {
                robot.position.0 -= max_len.0;
            }
            robot_map[robot.position.1 as usize][robot.position.0 as usize] = 'X';
        }
    }
    "Not implemented yet!".to_string()
}

fn display_tree(robot_map: &Vec<Vec<char>>) {
    for row in robot_map.iter() {
        for ch in row.iter() {
            print!("{ch}");
        }
        print!("\n");
    }
}

#[derive(Debug)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

fn handle_input(input: &str, seconds: isize, max_len: (isize, isize)) -> Vec<Robot> {
    let re = Regex::new(r"p=(-?\d*),(-?\d*) v=(-?\d*),(-?\d*)").unwrap();
    let mut output: Vec<Robot> = Vec::new();
    for line in input.lines() {
        for (_, [x, y, v_x, v_y]) in re.captures_iter(line).map(|c| c.extract()) {
            let mut x: isize = x.parse().unwrap();
            let mut y: isize = y.parse().unwrap();
            let v_x: isize = v_x.parse().unwrap();
            let v_y: isize = v_y.parse().unwrap();
            x = (x + v_x * seconds) % max_len.0;
            y = (y + v_y * seconds) % max_len.1;
            if x.is_negative() {
                x += max_len.0;
            }
            if y.is_negative() {
                y += max_len.1;
            }

            output.push(Robot {
                position: (x, y),
                velocity: (v_x, v_y),
            });
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "12");
    }

    // #[test]
    // fn test_2() {
    //     assert_eq!(puzzle_2(&INPUT), "");
    // }
}
