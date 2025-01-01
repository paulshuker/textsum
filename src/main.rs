use std::env;
use std::io::{self};

mod display;
mod word_counter;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No text given");
        return Ok(());
    }

    // The entire block of text is extracted into once string.
    let mut full_text: String = String::new();
    full_text.push_str(&args[1]);
    for i in 2..args.len() {
        full_text.push(' ');
        full_text.push_str(args[i].as_str());
    }

    // Words are extracted.
    let words: Vec<String> = full_text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let (common_words, common_counts) = word_counter::find_common_words(&words, 2);

    let mut title: String = String::from("Input: ");
    title.push_str(&full_text);

    let lines = display::get_display(&title, &common_words, &common_counts, &60);
    for line in lines {
        println!("{}", line);
    }

    return Ok(());
}
