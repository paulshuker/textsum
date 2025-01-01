use std::iter;

const SUFFIX_DEFAULT: &str = "...";

/// Centre the text.
fn centre_text(text: &String, max_width: &usize) -> String {
    let character_count = text.chars().count();
    if character_count == *max_width {
        return String::from(text);
    } else if character_count > *max_width {
        return limit_width(text, max_width, None);
    }

    let whitespace_count: f32 = ((*max_width - character_count) as f32) / 2_f32;
    let whitespace_count: usize = whitespace_count.floor() as usize;

    let mut output = String::new();
    for _ in 0..whitespace_count {
        output.push(' ');
    }
    output.push_str(text);
    for _ in 0..(max_width - character_count - whitespace_count) {
        output.push(' ');
    }

    return output;
}

fn limit_width(text: &String, max_width: &usize, suffix: Option<&String>) -> String {
    let mut suffix_copy = String::from(SUFFIX_DEFAULT);
    match suffix {
        Some(value) => suffix_copy = String::from(value),
        None => (),
    }
    let suffix_char_count = suffix_copy.chars().count();
    assert!(*max_width > suffix_char_count);

    let mut characters = text.chars().clone();
    if characters.clone().count() <= *max_width {
        return String::from(text);
    }

    let mut output = String::new();
    for _ in 0..(max_width - suffix_char_count) {
        match characters.next() {
            Some(value) => output.push(value),
            None => panic!("This should not happen"),
        }
    }

    output.push_str(&suffix_copy);
    return output;
}

pub fn get_display(
    title: &String,
    common_words: &Vec<String>,
    common_counts: &Vec<usize>,
    max_width: &usize,
) -> Vec<String> {
    let mut output = vec![];
    let row_seperator = iter::repeat("=").take(*max_width).collect::<String>();

    output.push(limit_width(title, max_width, None));
    output.push(row_seperator.clone());
    let mut line_centre = String::from("|");
    line_centre.push_str(&limit_width(
        &centre_text(
            &String::from("Most common words/phrases:"),
            &(*max_width - 2),
        ),
        &(*max_width - 2),
        None,
    ));
    line_centre.push('|');
    output.push(line_centre);
    output.push(row_seperator.clone());

    let mut longest_word: usize = 5;
    for word in common_words {
        let word_char_count = word.chars().count();
        if word_char_count > longest_word {
            longest_word = word_char_count;
        }
    }

    let mut word: String;
    let mut count: usize;
    for i in 0..common_words.len() {
        match common_words.get(i) {
            Some(value) => word = String::from(value),
            None => panic!(""),
        }
        match common_counts.get(i) {
            Some(value) => count = *value,
            None => panic!(""),
        }

        let mut line_centre = word;
        line_centre.push_str(" (");
        line_centre.push_str(&count.to_string());
        line_centre.push_str(")");
        line_centre = limit_width(&line_centre, &(max_width - 2), None);
        line_centre = centre_text(&line_centre, &(max_width - 2));
        let mut next_line = String::new();
        next_line.push('|');
        next_line.push_str(&line_centre);
        next_line.push('|');

        output.push(next_line);
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::{centre_text, limit_width, SUFFIX_DEFAULT};

    #[test]
    fn test_centre_text() {
        assert_eq!(centre_text(&String::from(""), &3), String::from("   "));
        assert_eq!(centre_text(&String::from("a"), &3), String::from(" a "));
        assert_eq!(centre_text(&String::from("a"), &4), String::from(" a  "));
        assert_eq!(centre_text(&String::from("ab"), &3), String::from("ab "));
        assert_eq!(centre_text(&String::from("abc"), &3), String::from("abc"));
        assert_eq!(centre_text(&String::from("ab"), &5), String::from(" ab  "));
        assert_eq!(centre_text(&String::from("abc"), &5), String::from(" abc "));
        assert_eq!(
            centre_text(&String::from("abcd edds"), &4),
            limit_width(&String::from("abcd edds"), &4, None)
        );
    }

    #[test]
    fn test_limit_width() {
        assert_eq!(
            limit_width(&String::from("abcd"), &10, None),
            String::from("abcd")
        );

        assert_eq!(
            limit_width(&String::from("abcd"), &4, None),
            String::from("abcd")
        );

        let mut expect = String::from("abcd e");
        expect.push_str(&SUFFIX_DEFAULT);
        assert_eq!(limit_width(&String::from("abcd efghi"), &9, None), expect);

        assert_eq!(
            limit_width(
                &String::from("abcd efghi"),
                &9,
                Option::from(&String::from("II"))
            ),
            String::from("abcd efII")
        );
        assert_eq!(
            limit_width(
                &String::from("abcd efghi"),
                &99,
                Option::from(&String::from("II"))
            ),
            String::from("abcd efghi")
        );
    }
}
