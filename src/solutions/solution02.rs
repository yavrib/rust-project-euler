pub fn solve() {
    let mut v: Vec<i32> = Vec::new();
    let mut iterator: i32 = 1;

    loop {
        match iterator {
            num if num < 4000000 => {
                let last_fib = match v.len() {
                    0 => 0,
                    n => v[n-1],
                };

                v.push(iterator);
                iterator = iterator + last_fib;
            },
            _ => break
        }
    }

    let sum = v.iter().fold(0, |accumulator, &item| match item % 2 == 0 { 
        true => accumulator + item,
        false => accumulator 
    });

    println!("Sum of the even fibonacci numbers that are below 4 millions");
    println!("The answer is {:?}", sum);
}
