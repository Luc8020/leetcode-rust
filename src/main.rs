mod problem1;

use std::io;

fn main() {
    let mut input = String::new();
    println!("Select the problem: \n 1) Median of Two Sorted Arrays");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.as_str().trim() {
        "1" => problem1::start(),
        _ => {}
    }
}
