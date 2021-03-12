use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn fetch_input(arguments: &Vec<String>) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(&arguments[1])?;
    let reader = std::io::BufReader::new(file);
    Ok(reader)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let reader = fetch_input(&args)?;

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    println!("{:?}", lines[0]);
    Ok(())
}
