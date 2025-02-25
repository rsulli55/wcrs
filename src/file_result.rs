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
