mod solutions;

use solutions::{ solution01 };
use std::io;

fn main() {
    println!("Greetings! This is an implementation of project euler written by Berkan Yavri in Rust.");

    let mut problem_number = String::new();

    io::stdin().read_line(&mut problem_number)
        .expect("Failed to read!");

    // Help! There needs to be a better way to do this!
    match problem_number.trim().as_ref() {
        "1" => solution01::solve(),
        //"2" => solution02::solve(),
        _ => println!("No such solution!")
    }
}
