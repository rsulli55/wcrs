use proptest::prop_compose;
use proptest::proptest;
use proptest::{
    char::{CharStrategy, DEFAULT_PREFERRED_RANGES, DEFAULT_SPECIAL_CHARS},
    prelude::prop,
    strategy,
    strategy::Strategy,
};

use crate::display_options::DisplayOptions;

#[derive(Debug, Eq, PartialEq)]
pub struct FileResult {
    pub bytes: usize,
    pub chars: usize,
    pub lines: usize,
    pub words: usize,
}

impl FileResult {
    pub fn new(bytes: usize, chars: usize, lines: usize, words: usize) -> Self {
        Self {
            bytes,
            chars,
            lines,
            words,
        }
    }
}

impl Default for FileResult {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl From<&FileResult> for [usize; 4] {
    fn from(value: &FileResult) -> Self {
        [value.lines, value.words, value.bytes, value.chars]
    }
}

pub fn file_result_string(result: &FileResult, options: &DisplayOptions) -> String {
    let options_arr: [bool; 4] = options.into();
    let result_arr: [usize; 4] = result.into();

    // TODO: I don't like this
    let mut s = String::new();
    options_arr.into_iter().enumerate().for_each(|(i, v)| {
        if v {
            s.push_str(&format!("{:<10}", &result_arr[i]));
        }
    });
    s
}

pub fn counts_for_line(line: &str) -> FileResult {
    // skip the first set of whitespace characters
    let start_of_word = match line.find(|c: char| !c.is_whitespace()) {
        Some(i) => i,
        None => {
            let chars = line.chars().count();
            let bytes = line.len();
            return FileResult::new(bytes, chars, 1, 0);
        }
    };
    let (whitespace_prefix, rest) = line.split_at(start_of_word);
    let (wchars, wbytes) = whitespace_prefix
        .chars()
        .fold((0, 0), |acc, c: char| (acc.0 + 1, acc.1 + c.len_utf8()));

    let (chars, bytes, words, _) = rest.chars().fold((0, 0, 0, true), |acc, c| {
        let chars = acc.0 + 1;
        let bytes = acc.1 + c.len_utf8();
        let mut prev_whitespace = acc.3;
        let curr_whitespace = c.is_whitespace();
        let incr_words = !curr_whitespace && prev_whitespace;
        let words = if incr_words { acc.2 + 1 } else { acc.2 };
        prev_whitespace = if prev_whitespace != curr_whitespace {
            curr_whitespace
        } else {
            prev_whitespace
        };

        (chars, bytes, words, prev_whitespace)
    });

    FileResult::new(wbytes + bytes, wchars + chars, 1, words)
}

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

fn word_data_strategy() -> impl strategy::Strategy<Value = WordData> {
    prop::collection::vec(proptest::char::any(), 1..20)
        // TODO: v.len() is giving the character count and String.len() gives the byte count
        .prop_flat_map(|v| {
            let chars = v.len();
            let word: String = v.into_iter().collect();
            let bytes = word.len();
            strategy::Just(WordData { word, chars, bytes })
        })
}

// make a custom CharStrategy which only selects whitespace
// Rust recognized whitespace: https://doc.rust-lang.org/reference/whitespace.html
fn whitespace_strategy() -> impl strategy::Strategy<Value = String> {
    let ranges = &['\t'..='\t', ' '..=' '];
    prop::collection::vec(
        CharStrategy::new_borrowed(&['\t', ' '], ranges, ranges),
        1..20,
    )
    .prop_flat_map(|v| {
        let spaces: String = v.iter().collect();
        strategy::Just(spaces)
    })
}

prop_compose! {
 fn line_data_strategy()
     (num_words in 1..20usize)
     (whitespaces in prop::collection::vec(
         whitespace_strategy(),
             // TODO: changes from num_words -1 because zip does will end once one iterator is exhausted
             num_words
     ),
     words in prop::collection::vec( word_data_strategy(), num_words),
     num_words in strategy::Just(num_words)) -> LineData {
     let chars = whitespaces.len() + words.len();
     let line: String = words.iter().zip(whitespaces.iter()).flat_map(|(w, s)| {
         w.word.chars().chain(s.chars())
     }).collect();
     let bytes = line.len();
     let words = num_words;

     LineData {line, words, chars, bytes }

 }
}

proptest! {
    #[test]
    fn test_word_strategy(wd in word_data_strategy()) {
        let word = &wd.word;
        let chars = &wd.chars;
        let bytes = &wd.bytes;
        dbg!("Word: {word}");
        assert_eq!(word.len(), *bytes);
    }

        #[test]
    fn test_line_strategy(ld in line_data_strategy()) {
        let line = &ld.line;
        let words= &ld.words;
        let chars = &ld.chars;
        let bytes = &ld.bytes;
        dbg!("Line: {line}");
        assert_eq!(line.len(), *bytes);
    }

}
