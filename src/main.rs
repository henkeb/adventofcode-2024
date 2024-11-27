use std::{error::Error, fs, path::Path};

use solutions::day01;

#[allow(dead_code)]
mod solutions;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", day01::puzzle_1(get_input(1)?));
    Ok(())
}

fn get_input(day: usize) -> Result<String, Box<dyn Error>> {
    let file_path = format!("./data/day{:0>2}.txt", day);
    if Path::new(&file_path).exists() {
        Ok(fs::read_to_string(&file_path)?)
    } else {
        // let cookie = fs::read_to_string(".session")?;
        let url = format!("https://adventofcode.com/{}/day/{}/input", 2023, day);

        let puzzle_input = reqwest::blocking::Client::new().get(url).send()?.text()?;

        fs::write(file_path, &puzzle_input)?;
        Ok(puzzle_input)
    }
}
