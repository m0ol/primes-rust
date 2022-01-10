use std::io;

fn main() {
    //let mut vet = vec![];
    //vet[0] = 20;
    //let mut v: [i32; 5] = [1, 2, 3, 4, 5];
    //v[3] = 10;
    let mut input = String::new();
    let mut maxinteration = String::new();
    let mut interation: i32;
    let mut numbers: Vec<&str>;
    io::stdin().read_line(&mut maxinteration).unwrap();
    io::stdin().read_line(&mut input).unwrap();

    interation = maxinteration.trim().parse::<i32>().unwrap();
    numbers = input.split(" ").collect();

    for i in 0..interation {
        for j in numbers[0].parse::<i32>().unwrap()..numbers[1].parse::<i32>().unwrap() {
            if j % numbers[0].parse::<i32>().unwrap() == 0 {
                println!("{}", j);
            }
        }
    }
    //println!("Is the number Correct? {}", numbers[0]);
    //println!("Is the number Correct2? {}", numbers[1]);
}
