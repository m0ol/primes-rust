use std::io;

mod prime_check {
    use std::io;
    pub fn prime_checkfn(n: i64) -> bool {
        let mut i: i64 = 2;
        let mut is_prime: bool = true;
        if n == 1 || n == 0 {
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

    pub fn user_input_primes() -> Vec<i64> {
        let mut input = String::new();
        let numbers: Vec<&str>;
        let mut numbersprimes: Vec<i64> = Vec::new();
        io::stdin().read_line(&mut input).unwrap();
        numbers = input.split(" ").collect();
        if numbers[0].trim().parse::<i64>().unwrap() < 1
            || numbers[1].trim().parse::<i64>().unwrap() > 1000000000
            || numbers[0].trim().parse::<i64>().unwrap() - numbers[1].trim().parse::<i64>().unwrap()
                > 100000
        {
            println!(
                "Please enter numbers between 1 and 1000000000 and with a difference of 100000"
            );
            return user_input_primes();
        }
        let mut i: i64 = numbers[0].trim().parse::<i64>().unwrap();
        while i <= numbers[1].trim().parse::<i64>().unwrap() {
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
    let interation: i64;
    let mut finalresult: Vec<Vec<i64>> = Vec::new();
    io::stdin().read_line(&mut maxinteration).unwrap();
    if maxinteration.trim().parse::<i64>().unwrap() > 10 {
        println!("Please enter a number less than 10");
        return main();
    }
    interation = maxinteration.trim().parse::<i64>().unwrap();
    for _i in 0..interation {
        finalresult.push(prime_check::user_input_primes());
    }
    println!("{:?}", finalresult);
}
