use std::collections::HashMap;

/// Find the n'th most common words.
///
/// If there are too many words of the same count, then they are prioritised by count
/// then by alphabetical order.
///
/// # Arguments
///
/// * `words` - A vector of strings containing every word.
/// * `n` - Integer of the top n most common words are returned.
///
/// # Returns
///
/// TupleType containing:
/// * `common_words` - Each most common word, from most to least common.
/// * `common_counts` - The number of times each common word was found.
pub fn find_common_words(words: &Vec<String>, n: usize) -> (Vec<String>, Vec<usize>) {
    if n <= 0 {
        panic!("n must be > 0, got {}", n);
    }

    let mut common_words: Vec<String> = Vec::new();
    let mut common_counts: Vec<usize> = Vec::new();
    if words.len() == 0 {
        return (common_words, common_counts);
    }

    // Create a word count for every unique word.
    let mut word_counts: HashMap<String, usize> = HashMap::new();
    let mut highest_word_count = 1;
    for word in words {
        let word_copy: String = word.clone();

        if !word_counts.contains_key(word) {
            word_counts.insert(word_copy, 1);
        } else {
            let mut new_count = 1;
            match word_counts.get(word) {
                Some(value) => new_count += value,
                None => panic!("This should never happen"),
            }
            word_counts.insert(word_copy, new_count);
            if new_count > highest_word_count {
                highest_word_count = new_count;
            }
        }
    }

    // Flip the hash map of word counts around. This way we can very quickly lookup all
    // words that have a specific count.
    let mut count_words: HashMap<usize, Vec<String>> = HashMap::new();
    for i in 0..highest_word_count + 1 {
        count_words.insert(i, vec![]);
    }
    for (word, count) in word_counts.into_iter() {
        match count_words.get_mut(&count) {
            Some(value) => value.push(word),
            None => panic!("This should never happen"),
        }
    }

    // Find the most common words.
    // We will keep dropping the threshold for the minimum word count until we have n
    // most common words or we run out of words. If we find more than n words, then the
    // words are taken in alphabetical order.
    let mut word_count_consider = highest_word_count + 1;
    while common_words.len() < n && word_count_consider > 0 {
        word_count_consider -= 1;

        let mut new_words: Vec<String> = vec![];
        match count_words.get_mut(&word_count_consider) {
            Some(value) => new_words = value.clone(),
            None => (),
        }
        if new_words.len() == 0 {
            continue;
        }

        // Sort new words alphabetically (case-insensitive).
        new_words.sort_by_key(|name| name.to_lowercase());

        // Remove words if too many words found.
        let mut remove_count = (common_words.len() + new_words.len()) as isize;
        remove_count -= n as isize;
        if remove_count > 0 {
            for _ in 0..remove_count {
                new_words.pop();
            }
        }

        // Append new words.
        let new_words_count = new_words.len();
        common_words.append(&mut new_words);
        common_counts.append(&mut vec![word_count_consider; new_words_count]);
    }

    return (common_words, common_counts);
}

#[cfg(test)]
mod tests {
    use super::find_common_words;

    #[test]
    fn test_find_common_words() {
        assert_eq!(find_common_words(&vec![], 1), (vec![], vec![]));
        assert_eq!(find_common_words(&vec![], 2), (vec![], vec![]));
        assert_eq!(find_common_words(&vec![], 3), (vec![], vec![]));
        assert_eq!(find_common_words(&vec![], 4), (vec![], vec![]));
        assert_eq!(
            find_common_words(&vec![String::from("Hi")], 1),
            (vec![String::from("Hi")], vec![1])
        );
        assert_eq!(
            find_common_words(&vec![String::from("Hi"); 2], 1),
            (vec![String::from("Hi")], vec![2])
        );
        assert_eq!(
            find_common_words(
                &vec![String::from("Hi"), String::from("hi"), String::from("Hi")],
                1
            ),
            (vec![String::from("Hi")], vec![2])
        );
        assert_eq!(
            find_common_words(
                &vec![
                    String::from("abc"),
                    String::from("abcd"),
                    String::from("abc"),
                    String::from("abc"),
                    String::from("abc"),
                    String::from("ba"),
                    String::from("ba"),
                    String::from("ba"),
                    String::from("ba"),
                ],
                1
            ),
            (vec![String::from("abc")], vec![4])
        );
        assert_eq!(
            find_common_words(
                &vec![
                    String::from("abc"),
                    String::from("abcd"),
                    String::from("abc"),
                    String::from("abc"),
                    String::from("abc"),
                    String::from("ba"),
                    String::from("ba"),
                    String::from("ba"),
                    String::from("ba"),
                ],
                2
            ),
            (vec![String::from("abc"), String::from("ba"),], vec![4, 4])
        );
        assert_eq!(
            find_common_words(
                &vec![
                    String::from("abc"),
                    String::from("abcd"),
                    String::from("abc"),
                    String::from("abc"),
                    String::from("abc"),
                    String::from("ba"),
                    String::from("ba"),
                    String::from("ba"),
                    String::from("ba"),
                ],
                3
            ),
            (
                vec![
                    String::from("abc"),
                    String::from("ba"),
                    String::from("abcd")
                ],
                vec![4, 4, 1]
            )
        );
    }
}
