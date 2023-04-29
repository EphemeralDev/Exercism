pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    match list.len() {
        1 => format!("And all for the want of a {}.", list[0]),
        2.. => {
            for i in 0..list.len() - 1 {
                proverb +=
                    format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]).as_str();
            }
            proverb + format!("And all for the want of a {}.", list[0]).as_str()
        }
        _ => proverb,
    }
}
