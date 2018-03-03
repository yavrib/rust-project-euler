pub fn find_next_prime(previous_prime: f64) -> f64 {
    let mut next_prime = previous_prime + 1f64;
    
    while !is_prime(next_prime) {
        next_prime = next_prime + 1f64;
    }

    return next_prime;
}

pub fn is_prime(next_prime: f64) -> bool {
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
