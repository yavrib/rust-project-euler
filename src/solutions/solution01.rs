pub fn solve() {
    let mut v: Vec<i32> = Vec::new();

    for i in 0..1000 {
        v.push(i);
    }

    let sum = v.iter().fold(0, |accumulator, &item| match item % 3 == 0 || item % 5 == 0 { 
        true => accumulator + item,
        false => accumulator 
    });

    println!("Sum of the numbers that are divisible by 3 or 5 till 1000");
    println!("The answer is {:?}", sum);
}
