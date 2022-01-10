pub fn PrimeCheckfn(n: i32) -> bool {
    let mut i: i32 = 2;
    let mut isPrime: bool = true;
    while i < n {
        if n % i == 0 {
            isPrime = false;
            break;
        }
        i += 1;
    }
    isPrime
}
