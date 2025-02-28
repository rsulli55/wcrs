/// Struct to hold which counts will be displayed to the user
/// based on command line options.
#[derive(Debug, PartialEq, Eq, Clone)]
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

    /// Join `self` with `other` by disjuncting their respective fields,
    /// mutating `self`.
    pub fn join_mut(&mut self, other: &Self) {
        self.lines = self.lines || other.lines;
        self.words = self.words || other.words;
        self.chars = self.chars || other.chars;
        self.bytes = self.bytes || other.bytes;
    }

    /// Join `self` with `other` by disjuncting their respective fields,
    /// returning the result.
    pub fn join(&self, other: &Self) -> Self {
        let mut res = self.clone();
        res.join_mut(other);
        res
    }

    /// Meet `self` with `other` by conjuncting their respective fields,
    /// mutating `self`.
    pub fn meet_mut(&mut self, other: &Self) {
        self.lines = self.lines && other.lines;
        self.words = self.words && other.words;
        self.chars = self.chars && other.chars;
        self.bytes = self.bytes && other.bytes;
    }

    /// Meet `self` with `other` by conjuncting their respective fields,
    /// returning the result.
    pub fn meet(&self, other: &Self) -> Self {
        let mut res = self.clone();
        res.meet_mut(other);
        res
    }

    /// Return the default display options for the `wc` command.
    pub fn default_options() -> Self {
        Self::new(true, true, false, true)
    }

    /// Returns `true` if all display options are off.
    pub fn all_off(&self) -> bool {
        !(self.lines || self.words || self.chars || self.bytes)
    }

    /// Create a `DisplayOption` with only lines on.
    pub fn with_lines_only() -> Self {
        Self::new(true, false, false, false)
    }

    /// Create a `DisplayOption` with only words on.
    pub fn with_words_only() -> Self {
        Self::new(false, true, false, false)
    }

    /// Create a `DisplayOption` with only chars on.
    pub fn with_chars_only() -> Self {
        Self::new(false, false, true, false)
    }

    /// Create a `DisplayOption` with only bytes on.
    pub fn with_bytes_only() -> Self {
        Self::new(false, false, false, true)
    }
}

impl Default for DisplayOptions {
    /// Return a `DisplayOptions` with all fields set to false.
    fn default() -> Self {
        Self::new(false, false, false, false)
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_all_off() {
        assert!(&DisplayOptions::default().all_off());
        assert_eq!(false, DisplayOptions::default_options().all_off());
    }

    #[test]
    fn test_meet() {
        let def_ops = DisplayOptions::default_options();
        let off_ops = DisplayOptions::default();
        let on_ops = DisplayOptions::new(true, true, true, true);
        assert_eq!(off_ops, off_ops.meet(&def_ops));
        assert_eq!(def_ops, on_ops.meet(&def_ops));
    }

    #[test]
    fn test_join() {
        let def_ops = DisplayOptions::default_options();
        let off_ops = DisplayOptions::default();
        let on_ops = DisplayOptions::new(true, true, true, true);
        assert_eq!(def_ops, off_ops.join(&def_ops));
        assert_eq!(on_ops, on_ops.join(&def_ops));
    }
}
