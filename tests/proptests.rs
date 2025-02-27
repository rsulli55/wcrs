use proptest::strategy::{Just, Strategy};
use proptest::{char, collection};
use proptest::{prop_compose, prop_oneof, proptest};

use rand::Rng;

use wcrs::file_result::{counts_for_file, counts_for_line};

#[derive(Debug, Clone)]
struct WordData {
    word: String,
    chars: usize,
    bytes: usize,
}

#[derive(Debug, Clone)]
struct LineData {
    line: String,
    words: usize,
    chars: usize,
    bytes: usize,
}

#[derive(Debug, Clone)]
struct FileData {
    file: String,
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
}

/// Produces a string of non-whitespace characters of length at most `max_length`
fn word_data_strategy(max_length: usize) -> impl Strategy<Value = WordData> {
    // char::any() can produce strings with whitespaces so we replace them with a
    // random alphanumeric character
    collection::vec(char::any(), 1..max_length).prop_flat_map(|mut v| {
        let mut rng = rand::rng();
        v.iter_mut().for_each(|c| {
            // TODO: use is_ascii_whitespace() instead?
            if c.is_whitespace() {
                *c = rng.sample(rand::distr::Alphanumeric) as char;
            }
        });
        let chars = v.len();
        let word: String = v.into_iter().collect();
        let bytes = word.len();
        Just(WordData { word, chars, bytes })
    })
}

/// Select either a tab '\t' or space ' ' with space weighted more heavily
fn space_tab_strategy() -> impl Strategy<Value = char> {
    prop_oneof![
        1 => Just('\t'),
        3 => Just(' '),
    ]
}

/// Produces a string of whitespace of length at most `max_length` that consist of either
/// tab '\t' or space ' '
fn whitespace_strategy(max_length: usize) -> impl Strategy<Value = String> {
    collection::vec(space_tab_strategy(), 1..max_length).prop_flat_map(|v| {
        let spaces: String = v.iter().collect();
        Just(spaces)
    })
}

/// Produces a string consisting of words separated by whitespace consisting of spaces and tabs.
// The string
/// has at must `max_num_components` words, words have length at most `max_word_length`,
/// and whitespaces between words have length at most `max_whitespace_length`.
fn line_data_strategy(
    max_num_components: usize,
    max_word_length: usize,
    max_whitespace_length: usize,
) -> impl Strategy<Value = LineData> {
    (
        collection::vec(word_data_strategy(max_word_length), max_num_components),
        collection::vec(
            whitespace_strategy(max_whitespace_length),
            max_num_components,
        ),
    )
        .prop_flat_map(|(wdv, spacesv)| {
            let words = wdv.len();
            let line = wdv
                .iter()
                .zip(spacesv.iter())
                .fold(String::new(), |acc, (wd, s)| acc + &wd.word + &s);

            // the spaces strings consist of space and tab characters, so its byte count and char count is equal
            let spaces_bytes = spacesv.iter().fold(0, |acc, s| acc + s.len());
            let chars = wdv.iter().fold(0, |acc, wd| acc + wd.chars) + spaces_bytes;
            let bytes = wdv.iter().fold(0, |acc, wd| acc + wd.bytes) + spaces_bytes;

            Just(LineData {
                line,
                words,
                chars,
                bytes,
            })
        })
}

/// Produces a string representing a file i.e. a string including multiple substrings
/// separated by newlines. The string will have at most `max_lines` newlines, each line
/// of the string consists of `max_num_components` words separated by whitespaces,
/// the words have size at most `max_word_length` and the whitespaces between
/// words have size at most `max_whitespace_length`.
fn file_data_strategy(
    max_lines: usize,
    max_num_components: usize,
    max_word_length: usize,
    max_whitespace_length: usize,
) -> impl Strategy<Value = FileData> {
    collection::vec(
        line_data_strategy(max_num_components, max_word_length, max_whitespace_length),
        1..max_lines,
    )
    .prop_flat_map(|v| {
        let words = v.iter().fold(0, |acc, l| acc + l.words);
        let lines = v.len();
        // we add a newline '\n' at the end of each line
        // so we need to add the number of lines to chars and bytes
        let chars = v.iter().fold(0, |acc, l| acc + l.chars) + lines;
        let bytes = v.iter().fold(0, |acc, l| acc + l.bytes) + lines;
        let file = v
            .iter()
            .fold(String::new(), |s: String, l| s + &l.line + "\n");
        Just(FileData {
            file,
            lines,
            words,
            chars,
            bytes,
        })
    })
}

proptest! {
    #[test]
    fn test_word_strategy(wd in word_data_strategy(10)) {
        let word = &wd.word;
        let chars = &wd.chars;
        let bytes = &wd.bytes;
        dbg!(word);
        assert_eq!(word.chars().count(), *chars);
        assert_eq!(word.len(), *bytes);
    }

    #[test]
    fn test_line_strategy(ld in line_data_strategy(10, 10, 5)) {
        let line = &ld.line;
        let words = &ld.words;
        let chars = &ld.chars;
        let bytes = &ld.bytes;
        dbg!(line);
        assert_eq!(line.split_ascii_whitespace().count(), *words);
        assert_eq!(line.chars().count(), *chars);
        assert_eq!(line.len(), *bytes);
    }

    #[test]
    fn test_file_strategy(fd in file_data_strategy(10, 10, 10, 5)) {
        let file = &fd.file;
        let lines = &fd.lines;
        let words = &fd.words;
        let chars = &fd.chars;
        let bytes = &fd.bytes;
        dbg!(file);
        assert_eq!(file.lines().count(), *lines);
        assert_eq!(file.split_ascii_whitespace().count(), *words);
        assert_eq!(file.chars().count(), *chars);
        assert_eq!(file.len(), *bytes);
    }

    #[test]
    fn test_counts_for_line(ld in line_data_strategy(30, 10, 5)) {
        let line = &ld.line;
        let words = &ld.words;
        let chars = &ld.chars;
        let bytes = &ld.bytes;
        let result = counts_for_line(&line);
        dbg!(line, &result);
        assert_eq!(result.words, *words);
        assert_eq!(result.chars, *chars);
        assert_eq!(result.bytes, *bytes);
    }

    #[test]
    fn test_counts_for_file(fd in file_data_strategy(30, 15, 10, 5)) {
        let file = &fd.file;
        let lines = &fd.lines;
        let words = &fd.words;
        let chars = &fd.chars;
        let bytes = &fd.bytes;
        let result = counts_for_file(&file);
        dbg!(file, &result);
        assert_eq!(result.lines, *lines);
        assert_eq!(result.words, *words);
        assert_eq!(result.chars, *chars);
        assert_eq!(result.bytes, *bytes);
    }

}
