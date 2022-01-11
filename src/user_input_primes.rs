use std::io;
mod prime_check;

pub fn user_input_primes() -> Vec<i32> {
    let mut input = String::new();
    let mut numbers: Vec<&str>;
    let mut numbersprimes: Vec<i32> = Vec::new();
    io::stdin().read_line(&mut input).unwrap();
    numbers = input.split(" ").collect();
    for _j in numbers[0].trim().parse::<i32>().unwrap()..numbers[1].trim().parse::<i32>().unwrap() {
        if prime_check::prime_checkfn(_j) {
            numbersprimes.push(_j);
        } else {
            continue;
        }
    }
    return numbersprimes;
}
