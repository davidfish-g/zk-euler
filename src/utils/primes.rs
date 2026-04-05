pub fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    let mut x = (n as f64).sqrt() as i64;
    while n % x != 0 {
        x -= 1;
    }
    x == 1
}

pub fn list_factors(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let sqrt = (n as f64).sqrt() as i64;
    for i in 1..sqrt {
        if n % i == 0 {
            factors.push(i);
            factors.push(n / i);
        }
    }
    factors
}
