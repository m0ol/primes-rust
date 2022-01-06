fn main() {
    use std::io;

    fn main() {
        //let mut vet = vec![];
        //vet[0] = 20;
        //let mut v: [i32; 5] = [1, 2, 3, 4, 5];
        //v[3] = 10;
        let mut input = String::new();
        let mut maxinteration = String::new();
        io::stdin().read_line(&mut maxinteration).unwrap();
        io::stdin().read_line(&mut input).unwrap();

        for i in 0..maxinteration.parse::<i32>().unwrap() {
            println!("Is the number Correct? {}", input);
        }
    }
}
