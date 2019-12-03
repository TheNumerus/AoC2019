use std::io::{BufReader, BufRead};
use std::fs::File;
use std::error::Error;

fn total_module_fuel(weight: u32) -> u32 {
    let mut module_weight = 0;
    let mut fuel = module_fuel(weight);
    module_weight += fuel;
    while fuel > 8 {
        let new_fuel = module_fuel(fuel);
        module_weight += new_fuel;
        fuel = new_fuel;
    }
    module_weight
}

fn module_fuel(weight: u32) -> u32 {
    (weight / 3) - 2
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input_file = File::open("./src/input_1")?;

    let buffer = BufReader::new(input_file);

    let mut total_fuel = 0;

    for line in buffer.lines() {
        let module_weight: u32 = line.unwrap().parse().unwrap();
        total_fuel += total_module_fuel(module_weight);
    }
    println!("Total fuel = {}", total_fuel);
    Ok(())
}