use std::io;

mod prime_check {
    use std::io;
    pub fn prime_checkfn(n: i32) -> bool {
        let mut i: i32 = 2;
        let mut is_prime: bool = true;
        if n == 1 {
            is_prime = false;
        }
        while i < n {
            if n % i == 0 {
                is_prime = false;
                break;
            }
            i += 1;
        }
        is_prime
    }

    pub fn user_input_primes() -> Vec<i32> {
        let mut input = String::new();
        let numbers: Vec<&str>;
        let mut numbersprimes: Vec<i32> = Vec::new();
        io::stdin().read_line(&mut input).unwrap();
        numbers = input.split(" ").collect();
        let mut i: i32 = numbers[0].trim().parse::<i32>().unwrap();
        while i <= numbers[1].trim().parse::<i32>().unwrap() {
            if prime_checkfn(i) {
                numbersprimes.push(i);
            }
            i += 1;
        }
        return numbersprimes;
    }
}

pub fn main() {
    let mut maxinteration = String::new();
    let interation: i32;
    let mut finalresult: Vec<Vec<i32>> = Vec::new();
    io::stdin().read_line(&mut maxinteration).unwrap();
    interation = maxinteration.trim().parse::<i32>().unwrap();
    for _i in 0..interation {
        finalresult.push(prime_check::user_input_primes());
    }
    println!("{:?}", finalresult);
}
