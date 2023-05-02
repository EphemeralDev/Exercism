use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        1.. => list
            .windows(2)
            .map(|word| format!("For want of a {} the {} was lost.\n", word[0], word[1]))
            .chain(once(format!("And all for the want of a {}.", list[0])))
            .collect(),

        _ => String::new(),
    }
}
