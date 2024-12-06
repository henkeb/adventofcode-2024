#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn value(&self) -> Coordinate {
        match *self {
            Self::North => Coordinate { x: 0, y: -1 },
            Self::East => Coordinate { x: 1, y: 0 },
            Self::South => Coordinate { x: 0, y: 1 },
            Self::West => Coordinate { x: -1, y: 0 },
        }
    }
}
#[derive(PartialEq)]
enum Tile {
    Object,
    Free,
    Outside,
}
#[derive(Debug, Clone, PartialEq)]
struct Coordinate {
    pub x: i32,
    pub y: i32,
}
struct Guard {
    position: Coordinate,
    direction: Direction,
    visited: Vec<Vec<bool>>,
    map: Vec<Vec<char>>,
    steps: usize,
}

impl Guard {
    fn new(map: Vec<Vec<char>>) -> Self {
        let start_pos = Self::find_start_pos(&map);
        let mut visited = vec![vec![false; map[0].len()]; map.len()];
        visited[start_pos.y as usize][start_pos.x as usize] = true;
        Guard {
            position: start_pos,
            direction: Direction::North,
            visited,
            map,
            steps: 0,
        }
    }

    fn find_start_pos(map: &Vec<Vec<char>>) -> Coordinate {
        let mut start_pos: Option<Coordinate> = None;
        for (i, row) in map.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                if *ch == '^' {
                    start_pos = Some(Coordinate {
                        x: j as i32,
                        y: i as i32,
                    });
                    break;
                }
            }
            if start_pos.is_some() {
                break;
            }
        }
        match start_pos {
            Some(pos) => pos,
            None => panic!["No start pos found!"],
        }
    }

    fn get_next_position(&self, dir: &Direction) -> Coordinate {
        let dir = dir.value();
        Coordinate {
            x: self.position.x + dir.x,
            y: self.position.y + dir.y,
        }
    }

    fn get_next_tile(&self, dir: &Direction) -> Tile {
        let next_pos = Self::get_next_position(&self, dir);
        if !next_pos.x.is_negative()
            && next_pos.x < self.map[0].len() as i32
            && !next_pos.y.is_negative()
            && next_pos.y < self.map.len() as i32
        {
            if self.map[next_pos.y as usize][next_pos.x as usize] == '#' {
                return Tile::Object;
            }
        } else {
            return Tile::Outside;
        }
        Tile::Free
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }

    fn step(&mut self) -> bool {
        match self.get_next_tile(&self.direction) {
            Tile::Outside => return false,
            Tile::Object => self.rotate(),
            Tile::Free => {
                self.position = self.get_next_position(&self.direction);
                self.visited[self.position.y as usize][self.position.x as usize] = true;
                self.steps += 1;
            }
        }
        true
    }

    fn get_unique_tiles(self) -> usize {
        self.visited.iter().flatten().filter(|&&val| val).count()
    }

    fn step2(&mut self) -> usize {
        let mut loops = 0;
        let mut rock_placement: Vec<Vec<bool>> =
            vec![vec![false; self.map[0].len()]; self.map.len()];

        loop {
            let start_dir = self.direction.clone();
            match self.get_next_tile(&start_dir) {
                Tile::Object => {}
                Tile::Outside => return loops,
                Tile::Free => {
                    let rock_pos = self.get_next_position(&start_dir);
                    if !rock_placement[rock_pos.y as usize][rock_pos.x as usize] {
                        let start_pos = self.position.clone();
                        self.map[rock_pos.y as usize][rock_pos.x as usize] = '#';
                        rock_placement[rock_pos.y as usize][rock_pos.x as usize] = true;
                        while self.step() {
                            if self.steps > 100000 {
                                loops += 1;
                                self.steps = 0;
                                break;
                            }
                        }
                        self.position = start_pos;
                        self.direction = start_dir;
                        self.map[rock_pos.y as usize][rock_pos.x as usize] = '.';
                    }
                }
            }
            self.step();
        }
    }
}
// Solution has
// Time complexity: O(?) not sure, depends on #paths taken
// Space complexity: O(n*m) row*col
pub fn puzzle_1(input: &str) -> String {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        map.push(line.chars().collect());
    }

    let mut guard = Guard::new(map);

    while guard.step() {
        // step
    }
    guard.get_unique_tiles().to_string()
}

// Solution has
// Time complexity: O(?) not sure, depends on #paths taken
// Space complexity: O(n*m) row*col
pub fn puzzle_2(input: &str) -> String {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        map.push(line.chars().collect());
    }

    let mut guard = Guard::new(map);
    guard.step2().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "41");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "6");
    }
}
