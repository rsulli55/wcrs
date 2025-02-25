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

type Word = String;

fn word_size_strategy() -> impl strategy::Strategy<Value = (Word, usize)> {
    prop::collection::vec(
        CharStrategy::new_borrowed(
            &DEFAULT_SPECIAL_CHARS,
            &DEFAULT_PREFERRED_RANGES,
            &DEFAULT_PREFERRED_RANGES,
        ),
        1..20,
    )
    // TODO: v.len() is giving the character count and String.len() gives the byte count
    .prop_flat_map(|v| {
        let len = v.len();
        (strategy::Just(v.into_iter().collect()), strategy::Just(len))
    })
}

proptest! {
    #[test]
    fn test_word_strategy((w, l) in word_size_strategy()) {
        dbg!("Word: {w}");
        assert_eq!(w.len(), l);
    }
}
