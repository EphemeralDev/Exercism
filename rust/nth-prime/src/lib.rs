pub fn nth(n: u32) -> u32 {
    let mut prime_count = 0;
    let mut candidate = 2;

    loop {
        if prime_count == n {
            return candidate;
        }
        candidate += 1;
        if primality_test(candidate) {
            prime_count += 1;
        }
    }
}

pub fn primality_test(n: u32) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    for i in (5..=(n as f32).sqrt() as u32).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }
    true
}
