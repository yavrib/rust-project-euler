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

fn find_next_prime(previous_prime: f64) -> f64 {
    let mut next_prime = previous_prime + 1f64;
    
    while !is_prime(next_prime) {
        next_prime = next_prime + 1f64;
    }

    return next_prime;
}

fn is_prime(next_prime: f64) -> bool {
    match next_prime as i64 {
        2 => {
            true
        },
        3 => {
            true
        },
        a => {
            let square_root: f64 = next_prime.sqrt().ceil();

            for number in 2..square_root as i64 {
                match (a as i64) % number == 0 {
                    true => false,
                    false => continue
                };
            }

            true
        }
    }
}
