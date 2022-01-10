pub fn prime_checkfn(n: i32) -> bool {
    let mut i: i32 = 2;
    let mut is_prime: bool = true;
    while i < n {
        if n % i == 0 {
            is_prime = false;
            break;
        }
        i += 1;
    }
    is_prime
}
