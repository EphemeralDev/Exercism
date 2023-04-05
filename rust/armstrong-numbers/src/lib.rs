pub fn num_to_digits(num: u32) -> Vec<u32> {
    let digits = num
        .to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect();
    digits
}

pub fn is_armstrong_number(num: u32) -> bool {
    let num_len = num.to_string().len() as u32;
    let mut sum: u32 = 0;
    let digits = num_to_digits(num);

    for i in digits {
        match sum.checked_add(i.pow(num_len)) {
            Some(checked) => sum = checked,
            None => return false,
        }
    }
    sum == num
}
