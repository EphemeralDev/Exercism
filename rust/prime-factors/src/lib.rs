//Naive solution from https://www.geeksforgeeks.org/prime-factor/

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factor: Vec<u64> = Vec::new();

    while n % 2 == 0 {
        factor.push(2);
        n = n / 2;
    }

    for i in (3..=(n as f64).sqrt() as u64).step_by(2) {
        while n % i == 0 {
            factor.push(i);
            n = n / i;
        }
    }
    if n > 2 {
        factor.push(n);
    }

    factor
}
