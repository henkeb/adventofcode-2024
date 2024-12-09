pub fn puzzle_1(input: &str) -> String {
    let (mut memory, _) = handle_input(input);
    let mut left = 0;
    let mut right = memory.len() - 1;
    while left < right {
        while memory[left] != None {
            left += 1;
        }
        while memory[right] == None {
            right -= 1;
        }
        memory.swap(left, right);
    }
    if memory[left].is_some() && memory[right].is_none() {
        memory.swap(left, right);
    }
    calculate_checksum(&memory).to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let (mut memory, mut id_to_move) = handle_input(input);
    let mut _right_start = memory.len() - 1;
    let mut right_end = memory.len() - 1;

    let mut free_memory = get_free_memory_blocks(&memory);
    while id_to_move != 0 {
        while memory[right_end] == None {
            right_end -= 1;
        }
        let current_id = memory[right_end].unwrap();
        _right_start = right_end;
        while memory[_right_start] != None && memory[_right_start].unwrap() == current_id {
            _right_start -= 1;
        }
        _right_start += 1;
        if id_to_move == memory[right_end].unwrap() {
            id_to_move -= 1;
            for mem in free_memory.iter_mut() {
                if mem.0 > mem.1 {
                    continue;
                }
                let free = mem.1 - mem.0 + 1;
                let curr = right_end - _right_start + 1;
                if free >= curr {
                    if _right_start > mem.0 {
                        for (i, j) in (_right_start..=right_end).enumerate() {
                            memory.swap(mem.0 + i, j);
                        }
                        mem.0 += curr;
                        break;
                    }
                }
            }
        }
        right_end = _right_start - 1;
    }
    calculate_checksum(&memory).to_string()
}

fn get_free_memory_blocks(memory: &Vec<Option<usize>>) -> Vec<(usize, usize)> {
    let mut free_start = 0;
    let mut _free_end = 0;
    let mut output = Vec::new();
    while free_start < memory.len() {
        while free_start < memory.len() && memory[free_start] != None {
            free_start += 1;
        }
        _free_end = free_start;
        while _free_end < memory.len() && memory[_free_end] == None {
            _free_end += 1
        }
        if _free_end != memory.len() - 1 {
            _free_end -= 1;
        }
        if free_start < memory.len() {
            output.push((free_start, _free_end));
        }
        free_start = _free_end + 1;
    }
    output
}

fn calculate_checksum(memory: &Vec<Option<usize>>) -> usize {
    let mut output = 0;
    for (i, val) in memory
        .iter()
        .enumerate()
        .filter(|(_i, val)| val.is_some())
        .map(|(i, val)| (i, val.unwrap()))
    {
        output += i * val;
    }
    output
}

fn handle_input(input: &str) -> (Vec<Option<usize>>, usize) {
    let mut memory: Vec<Option<usize>> = Vec::new();
    let mut id = 0;
    for (i, val) in input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, val)| (i, val.to_digit(10).unwrap()))
    {
        if i % 2 == 0 {
            for _ in 0..val {
                memory.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..val {
                memory.push(None);
            }
        }
    }
    if let Some(_id_last) = memory.iter().last() {
        id -= 1;
    }
    (memory, id)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_1() {
        assert_eq!(puzzle_1(&INPUT), "1928");
    }

    #[test]
    fn test_2() {
        assert_eq!(puzzle_2(&INPUT), "2858");
    }
}
