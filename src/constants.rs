pub const EXIT_SUCCESS: u8 = 0;
pub const EXIT_FAILURE: u8 = 1;
pub const PROGRAM: &str = "wcrs";
pub const USAGE: &str = "wcrs [OPTION]... [FILE]...";
pub const HELP: &str = concat!(
    "Print newline, word, and byte counts for each FILE, and a total \n\
    line if more than one FILE is specified. A word is a nonempty \n\
    sequence of non whitespace delimited by whitespace characters \n\
    or by start or end of input.\n\n\
    If no FILE is provided, or when FILE is -, read standard input.\n\n\
    The options below control which counts are printed, always in the
    order: newline, word, character, byte. \n",
    "  -c, --bytes          print the byte counts\n",
    "  -m, --chars          print the character counts\n",
    "  -l, --lines          print the newline counts\n",
    "  -w, --words          print the word counts\n",
    "  -h, --help           display this help and exit\n"
);
