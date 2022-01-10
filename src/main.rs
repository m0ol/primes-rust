use std::io;
mod prime_check;

fn main() {
    let mut input = String::new();
    let mut maxinteration = String::new();
    let interation: i32;
    let mut numbers: Vec<&str>;
    let mut numbersprimes: Vec<i32> = Vec::new();
    io::stdin().read_line(&mut maxinteration).unwrap();

    interation = maxinteration.trim().parse::<i32>().unwrap();

    for _i in 0..interation {
        io::stdin().read_line(&mut input).unwrap();
        numbers = input.split(" ").collect();
        for _j in
            numbers[0].trim().parse::<i32>().unwrap()..numbers[1].trim().parse::<i32>().unwrap()
        {
            if prime_check::prime_checkfn(_j) {
                numbersprimes.push(_j);
            } else {
                continue;
            }
        }
        numbers.clear();
    }
    println!("{:?}", numbersprimes);
}
