use crate::display_options::DisplayOptions;

/// Stores line, word, character, and byte counts for a file
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FileResult {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: usize,
}

impl FileResult {
    pub fn new(lines: usize, words: usize, chars: usize, bytes: usize) -> Self {
        Self {
            lines,
            words,
            chars,
            bytes,
        }
    }

    /// Add the counts in `self` and `other`, storing
    /// the results in `self`
    pub fn add_mut(&mut self, other: &Self) {
        self.lines += other.lines;
        self.words += other.words;
        self.chars += other.chars;
        self.bytes += other.bytes;
    }

    /// Add the counts in `self` and `other` returning the result
    pub fn add(&self, other: &Self) -> Self {
        let mut res = self.clone();
        res.add_mut(other);
        res
    }
}

impl Default for FileResult {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

/// Convert a FileResult to usize array of length 4
impl From<&FileResult> for [usize; 4] {
    fn from(value: &FileResult) -> Self {
        [value.lines, value.words, value.chars, value.bytes]
    }
}

/// Produce a string representation of `result` only displaying the
/// counts for fields turned on in `options`
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

/// Compute line, word, character, and byte counts for `s`.
/// Assumes that `s` does not have any newline characters.
pub fn counts_for_line(s: &str) -> FileResult {
    // skip the first set of whitespace characters
    // TODO: use is_ascii_whitespace() instead?
    let start_of_word = match s.find(|c: char| !c.is_whitespace()) {
        Some(i) => i,
        None => {
            let chars = s.chars().count();
            let bytes = s.len();
            return FileResult::new(0, 0, chars, bytes);
        }
    };
    let (whitespace_prefix, rest) = s.split_at(start_of_word);
    let (wchars, wbytes) = whitespace_prefix
        .chars()
        .fold((0, 0), |acc, c: char| (acc.0 + 1, acc.1 + c.len_utf8()));

    let (chars, bytes, words, _) = rest.chars().fold((0, 0, 0, true), |acc, c| {
        let chars = acc.0 + 1;
        let bytes = acc.1 + c.len_utf8();
        let prev_whitespace = acc.3;
        // TODO: use is_ascii_whitespace() instead?
        let curr_whitespace = c.is_whitespace();
        // if the prev character was a whitespace and the current character is not
        // increment the word count
        let incr_words = !curr_whitespace && prev_whitespace;
        let words = if incr_words { acc.2 + 1 } else { acc.2 };

        (chars, bytes, words, curr_whitespace)
    });

    FileResult::new(0, words, wchars + chars, wbytes + bytes)
}

/// Compute line, word, character, and byte counts for `file`.
pub fn counts_for_file(file: &str) -> FileResult {
    // TODO: Should we handle \r\n newlines as well?
    file.split_inclusive('\n')
        .fold(FileResult::default(), |acc, l| {
            let line_result = counts_for_line(l);
            FileResult::new(
                acc.lines + 1,
                acc.words + line_result.words,
                acc.chars + line_result.chars,
                acc.bytes + line_result.bytes,
            )
        })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_file_result_string() {
        let result = FileResult::new(1, 2, 3, 4);
        let options = DisplayOptions::new(true, false, true, false);

        assert_eq!(
            &file_result_string(&result, &options),
            "1         3         "
        );
    }

    #[test]
    fn test_add() {
        let fr1 = FileResult::new(1, 2, 3, 4);
        let fr2 = FileResult::new(3, 2, 5, 9);
        assert_eq!(fr1.add(&fr2), FileResult::new(4, 4, 8, 13));
    }
}
