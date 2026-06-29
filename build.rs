use std::{env, fs};
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let test_dir = Path::new(&out_dir).join("test.rs");
    let contents = "pub static test_value: u32 = 5;";
    fs::write(test_dir, contents).unwrap();
}

fn parse_input() {

}