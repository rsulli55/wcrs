#[derive(Debug, PartialEq)]
pub struct DisplayOptions {
    bytes: bool,
    chars: bool,
    lines: bool,
    words: bool,
}
impl DisplayOptions {
    pub fn new(bytes: bool, chars: bool, lines: bool, words: bool) -> Self {
        Self {
            bytes,
            chars,
            lines,
            words,
        }
    }
}

// TODO: fix the different order, it has to do with printing like wc
impl From<&DisplayOptions> for [bool; 4] {
    fn from(value: &DisplayOptions) -> Self {
        [value.lines, value.words, value.bytes, value.chars]
    }
}

pub fn num_to_display(options: &DisplayOptions) -> u8 {
    Into::<[bool; 4]>::into(options)
        .into_iter()
        .fold(0u8, |acc, b| if b { acc + 1 } else { acc })
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self::new(true, false, true, true)
    }
}
