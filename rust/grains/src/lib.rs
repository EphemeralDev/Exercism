pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => (2_u64).pow(s - 1).into(),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..=64).map(|x| 2_u64.pow(x - 1)).sum()
}
