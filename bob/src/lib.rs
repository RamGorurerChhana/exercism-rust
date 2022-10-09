pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message == "" {
        return "Fine. Be that way!";
    }

    let question = message.ends_with("?");
    let uppercase = message.contains(char::is_alphabetic) && message == message.to_uppercase();

    match (uppercase, question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (false, true) => "Sure.",
        (true, false) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
