pub fn square_of_sum(n: u32) -> u32 {
    //A couple explanations to help understand why the turbofish is necessary
    //https://matematikaadit.github.io/posts/rust-turbofish.html
    //https://users.rust-lang.org/t/turbofish-operator/53921
    (1..=n).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
