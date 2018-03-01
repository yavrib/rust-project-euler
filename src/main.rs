mod solutions;

use solutions::{ solution01, solution02 };
use std::io;

fn main() {
    println!("Greetings! This is an implementation of project euler written by Berkan Yavri in Rust.");
    println!("Enter the number of solution to see the result.");
    println!("Type \"exit\" to exit.");

    loop {
        let mut problem_number = String::new();

        io::stdin().read_line(&mut problem_number)
            .expect("Failed to read!");

        // Help! There needs to be a better way to do this!
        match problem_number.trim().as_ref() {
            "1" => solution01::solve(),
            "2" => solution02::solve(),
            "exit" => {
                println!("The program will exit now.");
                break;
            },
            any => println!("If you were looking for a solution with the name \"{}\", there is no such solution.", any)
        }
    }
}
