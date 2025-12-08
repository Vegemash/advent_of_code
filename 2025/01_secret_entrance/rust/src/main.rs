use rust::{calculate_code, calculate_code2};
use std::path::PathBuf;

fn main() {
    // The original script used a relative path `../input1.txt`.
    // We keep the same behaviour – the binary is expected to be run from
    // `mycode/target/debug` (or `release`), so `../input1.txt` points to the
    // file next to the Python version.
    let input_path = PathBuf::from("../input1.txt");

    println!("first method:  {}", calculate_code(&input_path));
    println!("second method: {}", calculate_code2(&input_path));
}
