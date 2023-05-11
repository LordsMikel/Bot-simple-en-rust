use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        print!("You: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let output = respond(input.trim());
        println!("Bot: {}", output);
    }
}

fn respond(input: &str) -> &str {
    match input {
        "hello" => "Hi there!",
        "how are you" => "I'm a bot, I don't have feelings, but thanks for asking!",
        "may the force be with you" => "And also with you.",
        "I am your father" => "No, that's not true! That's impossible!",
        "do or do not" => "There is no try.",
        "Hello there" => "General Kenobi!",
        _ => "Sorry, I didn't understand that.",
    }
}
