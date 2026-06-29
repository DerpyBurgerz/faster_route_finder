mod parser;

use std::{env, fs};
use std::path::Path;

const INPUT_LENGTH: usize = 1177;

#[derive(Default, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}
#[derive(Debug)]
struct InputEntry {
    order: u16,
    town: String,
    frequency: u8,
    container_count: u8,
    container_volume: u16,
    emptying_time: u32,
    matrix_id: u16,
    coordinate: Coordinate,
}
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let test_dir = Path::new(&out_dir).join("test.rs");
    let contents = "pub static test_value: u32 = 5;";
    parse_input();
    fs::write(test_dir, contents).unwrap();

}

fn parse_input() -> [InputEntry; INPUT_LENGTH]{
    let input_data = Path::new("input_data.txt");
    let contents = fs::read_to_string(input_data)
        .expect("There should be a file named input_data.txt which contains all the locations and stuff");



    let input: Vec<InputEntry> = contents
        .lines()
        .skip(1)
        .enumerate()
        .map(|(i, line)| {

            println!("{line}");
            let mut values = line.split(";");

            InputEntry{
                order: values.next().unwrap().parse().expect("failed to parse order"),
                town: values.next().unwrap().to_string(),
                frequency: values.next().unwrap()[0..1].parse().expect("failed to parse frequency"),
                container_count: values.next().unwrap().parse().expect("failed to parse container_count"),
                container_volume: values.next().unwrap().parse().expect("failed to parse container_volume"),
                emptying_time: values.next().unwrap().parse().expect("failed to parse emptying_time"),
                matrix_id: values.next().unwrap().parse().expect("failed to parse matrix_id"),
                coordinate: Coordinate{
                    x: values.next().unwrap().parse().expect("failed to parse x coordinate"),
                    y: values.next().unwrap().parse().expect("failed to parse y coordinate"),
                }
            }
        }).collect();

    assert_eq!(input.len(), INPUT_LENGTH);

    input.try_into().expect("failed to put vec into array")
}