use std::env;
use std::fs;
use std::io;
use std::path;

mod counter;
mod display;

const DEFAULT_WIDTH: usize = 80;
const DEFAULT_COMMON_COUNT: usize = 9;

fn run_main_with_args(args: Vec<String>) -> io::Result<()> {
    if args.len() < 2 {
        println!("No text given");
        return Ok(());
    }
    if args.len() > 2 {
        println!("Can only give one argument, got {}", args.len() - 1);
        return Ok(());
    }

    let file_path = path::Path::new(&args[1]).canonicalize();
    let given_file: bool = !file_path.is_err();

    let mut full_text: String;
    if given_file {
        let file_path = file_path.unwrap();
        full_text =
            fs::read_to_string(file_path).expect("Failed to read given file path");
    } else {
        // The entire block of text is extracted into once string.
        full_text = String::new();
        full_text.push_str(&args[1]);
        for i in 2..args.len() {
            full_text.push(' ');
            full_text.push_str(args[i].as_str());
        }
    }

    let (words, char_count, numeric_count, whitespace_count) =
        counter::count_characters(&full_text);

    let word_count = words.len();
    let (common_words, common_counts) =
        counter::find_common_words(&words, DEFAULT_COMMON_COUNT);

    let lines = display::get_display(
        &word_count,
        &char_count,
        &numeric_count,
        &whitespace_count,
        &common_words,
        &common_counts,
        &DEFAULT_WIDTH,
    );
    for line in lines {
        println!("{}", line);
    }

    return Ok(());
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    return run_main_with_args(args);
}

#[cfg(test)]
mod tests {
    use super::run_main_with_args;

    #[test]
    fn test_run_main_with_args() {
        let args: Vec<String> = vec![String::from(""), String::from("abc a")];
        let result = run_main_with_args(args);
        assert!(!result.is_err(), "Unexpected result error: {:?}", result);

        let args: Vec<String> = vec![String::from(""), String::from("abc abc abc")];
        let result = run_main_with_args(args);
        assert!(!result.is_err(), "Unexpected result error: {:?}", result);

        let args: Vec<String> =
            vec![String::from("abc"), String::from("a b c d e f g h i j k l")];
        let result = run_main_with_args(args);
        assert!(!result.is_err(), "Unexpected result error: {:?}", result);
    }
}
