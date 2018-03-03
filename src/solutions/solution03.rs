use common_operations::prime::{ find_next_prime };
use std::time::Instant;

pub fn solve() {
    // Start from 2 and detect if the number is prime, if so check if it is divisible by the target. If it is divisible, then divide the target. Then take the last number that is not 1. Last number standing is the answer. 
    let start_time = Instant::now();
    let mut target = 600851475143f64;
    let mut highest_prime_multiplier = 2f64;

    loop {
        match target % highest_prime_multiplier == 0f64 {
            true => {
                if target == highest_prime_multiplier {
                    break;
                } else {
                    target = target/highest_prime_multiplier;
                }
            },
            false => highest_prime_multiplier = find_next_prime(highest_prime_multiplier)
        }
    }

    let elapsed = start_time.elapsed();
    println!("The largest prime factor of the number {} should be found in under 60 seconds", target);
    println!(
        "The answer is {:?}, calculation completed in 0.{seconds:03} seconds or {milliseconds:?} milliseconds.", 
        target as i64, seconds = (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64, 
        milliseconds = (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64
    );
}
