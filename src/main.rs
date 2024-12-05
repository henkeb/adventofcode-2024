use std::io::Write;
use std::{error::Error, fs, io::BufRead, path::Path};

use solutions::day05;

#[allow(dead_code)]
mod solutions;

fn main() -> Result<(), Box<dyn Error>> {
    let setup: bool = match std::env::args().nth(2) {
        Some(val) if val == "t" => true,
        Some(..) => false,
        None => false,
    };
    match std::env::args()
        .nth(1)
        .expect("no day is given")
        .parse::<usize>()?
    {
        day if day >= 1 && day <= 25 => {
            if setup {
                create_daily_file(day)?;
                get_input(day)?;
                add_mod(day)?;
            } else {
                println!("Puzzle 1: {}", day05::puzzle_1(&get_input(day)?));
                println!("Puzzle 2: {}", day05::puzzle_2(&get_input(day)?));
            }
        }
        _ => println!("Wrong input!"),
    }
    Ok(())
}

fn create_daily_file(day: usize) -> Result<(), Box<dyn Error>> {
    let file_path = format!("./src/solutions/day{:0>2}.rs", day);
    let template_path = format!("./template/template.rs");
    if !Path::new(&file_path).exists() {
        fs::copy(&template_path, &file_path)?;
    }
    Ok(())
}

fn get_input(day: usize) -> Result<String, Box<dyn Error>> {
    let file_path = format!("./data/day{:0>2}.txt", day);
    if Path::new(&file_path).exists() {
        Ok(fs::read_to_string(&file_path)?)
    } else {
        let url = format!("https://adventofcode.com/{}/day/{}/input", 2024, day);

        let puzzle_input = reqwest::blocking::Client::new()
            .get(url)
            .header("Cookie", "session=EnterYourCookieHere")
            .send()?
            .text()?;

        fs::write(file_path, &puzzle_input)?;
        Ok(puzzle_input)
    }
}

fn add_mod(day: usize) -> Result<(), Box<dyn Error>> {
    let file_path = "./src/solutions/mod.rs";
    let file = std::fs::File::open(&file_path)?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        if line? == format!("pub mod day{:0>2};", day) {
            return Ok(());
        }
    }

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{}", format!("pub mod day{:0>2};", day))?;

    Ok(())
}
