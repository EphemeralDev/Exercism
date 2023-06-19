pub fn reply(message: &str) -> &str {
    let msg = message.trim_end();

    if msg.is_empty() {
        return "Fine. Be that way!";
    }

    match (check_question(msg), check_yelling(msg)) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Sure.",
        (_, true) => "Whoa, chill out!",
        (_, _) => "Whatever.",
    }
}

pub fn check_question(msg: &str) -> bool {
    if msg.ends_with("?") {
        true
    } else {
        false
    }
}

pub fn check_yelling(msg: &str) -> bool {
    if msg == msg.to_ascii_uppercase() && msg.chars().any(|ch| ch.is_alphabetic()) {
        true
    } else {
        false
    }
}
