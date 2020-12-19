// fn is_question. ends_with("?")
// fn is_sentence 
// fn is_YELL is str.make_ascii_uppercase ==
// fn say nothing filter on whitespace

pub fn is_question(message: &str) -> bool {

    message.ends_with("?")
}

pub fn is_yell(message: &str) -> bool {
    let message_cmp = message.to_ascii_uppercase();
    let diff = message_cmp.chars().zip(message.chars()).filter(|&(a, b)| a != b).collect::<Vec<_>>();
    diff.len() == 0 && is_intelligible(message)

}

pub fn is_void(message: &str) -> bool {
    message.len() == 0
}

pub fn is_intelligible(message: &str) -> bool {
    let diff = message.chars().filter(|c| c.is_alphabetic()).collect::<String>();
    !diff.is_empty()
}

pub fn trial(message: &str) -> &str {
    let s_message = message.trim_end_matches(char::is_whitespace);
    s_message
}

pub fn reply(message: &str) -> &str {
    let s_message = message.trim();
    match (is_question(s_message), is_yell(s_message), is_void(s_message), is_intelligible(s_message)) {
        (true, false, false, _) =>  "Sure.",
        (false, true, false, true) =>  "Whoa, chill out!",
        (true, true, false, true) =>  "Calm down, I know what I'm doing!",
        (false, _, true, _) =>  "Fine. Be that way!",
        _ =>  "Whatever."
    }
}
