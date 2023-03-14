use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();

    for word in magazine {
        *map.entry(word).or_insert(0) += 1;
    }

    for word in note {
        *map.entry(word).or_insert(0) -= 1;

        if map.values().any(|&k| k < 0) {
            return false;
        }
    }
    true
}
