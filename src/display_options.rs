/// Struct to hold which counts will be displayed to the user
/// based on command line options.
#[derive(Debug, PartialEq)]
pub struct DisplayOptions {
    lines: bool,
    words: bool,
    chars: bool,
    bytes: bool,
}
impl DisplayOptions {
    pub fn new(lines: bool, words: bool, chars: bool, bytes: bool) -> Self {
        Self {
            lines,
            words,
            chars,
            bytes,
        }
    }
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self::new(true, true, false, true)
    }
}

/// Convert `options` to a bool array of length 4.
impl From<&DisplayOptions> for [bool; 4] {
    fn from(options: &DisplayOptions) -> Self {
        [options.lines, options.words, options.chars, options.bytes]
    }
}

/// Calculate how many options are turned on.
pub fn num_to_display(options: &DisplayOptions) -> u8 {
    Into::<[bool; 4]>::into(options)
        .into_iter()
        .fold(0u8, |acc, b| if b { acc + 1 } else { acc })
}
