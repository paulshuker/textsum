use std::iter;

const SUFFIX_DEFAULT: &str = "...";
const DEFAULT_PROPORTIONS: (f32, f32) = (7_f32, 3_f32);
const COMMON_SUBHEADING: &str = "Most common words/phrases";
const COUNT_SUBHEADING: &str = "Counts";

/// Place commas after three digits for human-readability.
///
/// # Arguments
///
/// * `number` - Number to comma separate.
///
/// # Returns
///
/// `number_formatted` - Number as a string with comma-separation.
fn comma_separate_number(number: &usize) -> String {
    return number
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",");
}

/// Centre the text.
fn centre_text(text: &String, max_width: &usize) -> String {
    let character_count = text.chars().count();
    if character_count == *max_width {
        return String::from(text);
    } else if character_count > *max_width {
        return limit_width(text, max_width, None);
    }

    let whitespace_count: f32 = ((*max_width - character_count) as f32) / 2_f32;
    let whitespace_count: usize = whitespace_count.ceil() as usize;

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
    word_count: &usize,
    char_count: &usize,
    numeric_count: &usize,
    whitespace_count: &usize,
    common_words: &Vec<String>,
    common_counts: &Vec<usize>,
    max_width: &usize,
) -> Vec<String> {
    let mut output = vec![];
    let row_seperator = iter::repeat("=").take(*max_width).collect::<String>();

    let mut common_word_phrases: Vec<String> = Vec::new();
    for (word, count) in iter::zip(common_words, common_counts) {
        common_word_phrases.push(
            word.clone()
                + &String::from(" (")
                + &comma_separate_number(&count)
                + &String::from(")"),
        );
    }

    let mut count_phrases: Vec<String> = Vec::new();
    count_phrases.push(String::from("Word: ") + &comma_separate_number(&word_count));
    count_phrases
        .push(String::from("Character: ") + &comma_separate_number(&char_count));
    count_phrases
        .push(String::from("Digit: ") + &comma_separate_number(&numeric_count));
    count_phrases
        .push(String::from("Whitespace: ") + &comma_separate_number(&whitespace_count));

    // Fill phrases with empty phrases if one vertical side is larger than the other.
    let fill_word_phrases: bool = common_word_phrases.len() < count_phrases.len();
    if common_word_phrases.len() != count_phrases.len() {
        for _ in
            0..(count_phrases.len() as isize - common_word_phrases.len() as isize).abs()
        {
            if fill_word_phrases {
                common_word_phrases.push(String::from(""));
            } else {
                count_phrases.push(String::from(""));
            }
        }
    }

    // First two lines.
    output.push(row_seperator.clone());

    let mut longest_common_phrase: usize = COMMON_SUBHEADING.chars().count();
    for phrase in common_words {
        let phrase_char_count = phrase.chars().count();
        if phrase_char_count > longest_common_phrase {
            longest_common_phrase = phrase_char_count;
        }
    }
    // Additional character for the vertical separator.
    longest_common_phrase += 1;
    let mut longest_count_phrase: usize = 5;
    for phrase in &count_phrases {
        if phrase.len() > longest_count_phrase {
            longest_count_phrase = phrase.len();
        }
    }

    // Find out how many characters to give to each vertical strip.
    let mut common_word_width = (*max_width - 2) as f32;
    common_word_width *=
        DEFAULT_PROPORTIONS.0 / (DEFAULT_PROPORTIONS.0 + DEFAULT_PROPORTIONS.1);
    let mut common_word_width: usize = common_word_width.ceil() as usize;
    let mut counts_width = *max_width - 2 - common_word_width;

    if common_word_width > longest_common_phrase && counts_width < longest_count_phrase
    {
        common_word_width = longest_common_phrase.min(longest_count_phrase);
        counts_width = *max_width - 2 - common_word_width;
    } else if common_word_width < longest_common_phrase
        && counts_width > longest_count_phrase
    {
        counts_width = longest_count_phrase.min(longest_common_phrase);
        common_word_width = *max_width - 2 - counts_width;
    }

    let mut headings_line = String::from("|");
    headings_line.push_str(&limit_width(
        &centre_text(&String::from(COMMON_SUBHEADING), &(common_word_width - 1)),
        &(common_word_width - 1),
        None,
    ));
    headings_line.push('|');
    headings_line.push_str(&limit_width(
        &centre_text(&String::from(COUNT_SUBHEADING), &counts_width),
        &counts_width,
        None,
    ));
    headings_line.push('|');
    output.push(headings_line);
    output.push(row_seperator.clone());

    for (common_phrase, count_phrase) in iter::zip(common_word_phrases, count_phrases) {
        let mut new_line: String = String::from("|");
        new_line += &limit_width(
            &centre_text(&common_phrase, &(common_word_width - 1)),
            &(common_word_width - 1),
            None,
        );
        new_line += "|";
        new_line += &limit_width(
            &centre_text(&count_phrase, &counts_width),
            &counts_width,
            None,
        );
        new_line += "|";
        output.push(new_line);
    }

    output.push(row_seperator);

    return output;
}

#[cfg(test)]
mod tests {
    use super::{centre_text, comma_separate_number, limit_width, SUFFIX_DEFAULT};

    #[test]
    fn test_comma_separate_number() {
        assert_eq!(comma_separate_number(&1), String::from("1"));
        assert_eq!(comma_separate_number(&10), String::from("10"));
        assert_eq!(comma_separate_number(&222), String::from("222"));
        assert_eq!(comma_separate_number(&1_000), String::from("1,000"));
        assert_eq!(comma_separate_number(&1_005_502), String::from("1,005,502"));
    }

    #[test]
    fn test_centre_text() {
        assert_eq!(centre_text(&String::from(""), &3), String::from("   "));
        assert_eq!(centre_text(&String::from("a"), &3), String::from(" a "));
        assert_eq!(centre_text(&String::from("a"), &4), String::from("  a "));
        assert_eq!(centre_text(&String::from("ab"), &3), String::from(" ab"));
        assert_eq!(centre_text(&String::from("abc"), &3), String::from("abc"));
        assert_eq!(centre_text(&String::from("ab"), &5), String::from("  ab "));
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
