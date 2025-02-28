use std::path::PathBuf;

use crate::constants::*;
use crate::display_options::DisplayOptions;

/// Print a message saying `arg` is not a recognized option and exit with
/// `EXIT_FAILURE`.
fn invalid_option_exit(arg: &str) {
    eprintln!("{}: unrecognized option {}", PROGRAM, arg);
    std::process::exit(EXIT_FAILURE as i32);
}

/// Parse `arg` looking for options  starting with `--`.
/// Returns a `DisplayOptions`
/// representing what option was requested. If `arg` is
/// `--help` the help message is printed and the program is exited.
/// If `arg` is not a known option, a message is printed to stderr
// and the program is exited.
// TODO: return an option or exit code instead and exit in parse_args or higher up?
fn parse_long_option(arg: &str) -> DisplayOptions {
    // if help was requested, print it and exit
    if arg == "--help" {
        println!("{}\n{}", USAGE, HELP);
        std::process::exit(EXIT_SUCCESS as i32);
    }
    match arg {
        "--lines" => DisplayOptions::with_lines_only(),
        "--words" => DisplayOptions::with_words_only(),
        "--chars" => DisplayOptions::with_chars_only(),
        "--bytes" => DisplayOptions::with_bytes_only(),
        _ => {
            invalid_option_exit(arg);
            unreachable!()
        }
    }
}

/// Parse `arg` looking for options  starting with `-`.
/// Returns a `DisplayOptions`
/// representing what option was requested. If `arg` is
/// `-h` the help message is printed and the program is exited.
/// If `arg` is not a known option, it is ignored and a mesage is printed
/// to stderr.
fn parse_short_option(arg: &str) -> DisplayOptions {
    // if help was requested, print it and exit
    if arg == "-help" {
        println!("{}\n{}", USAGE, HELP);
        std::process::exit(EXIT_SUCCESS as i32);
    }
    match arg {
        "-l" => DisplayOptions::with_lines_only(),
        "-w" => DisplayOptions::with_words_only(),
        // `-m` is chars
        "-m" => DisplayOptions::with_chars_only(),
        // `-c` is bytes
        "-c" => DisplayOptions::with_bytes_only(),
        _ => {
            invalid_option_exit(arg);
            unreachable!()
        }
    }
}
/// Parse the command lines arguments and return a tuple consisting
/// of the display options, the paths to perform counting on, and
/// whether or not to also read from stdin.
pub fn parse_args(args: &[String]) -> (DisplayOptions, Vec<PathBuf>, bool) {
    let mut cli_option_seen = false;
    let mut read_stdin = false;
    let mut display_options = DisplayOptions::default();
    let mut paths: Vec<PathBuf> = Vec::new();
    for arg in args {
        // check if `arg` starts with `-`
        if &arg[0..=0] == "-" {
            // if the argument length is at least two then we have a
            // a potential CLI option
            if arg.len() >= 2 {
                // we can set this to true because if arg isn't a valid
                // CLI option, we exit the program
                cli_option_seen = true;
                if &arg[0..=1] == "--" {
                    display_options.join_mut(&parse_long_option(arg));
                } else {
                    display_options.join_mut(&parse_short_option(arg));
                }
            }
            // otherwise `arg` is `-` so we need to read stdin
            else {
                read_stdin = true;
            }
        }
        // otherwise we have a potential path to read
        else {
            paths.push(PathBuf::from(arg));
        }
    }

    // if we never saw a CLI option use the wc default options
    if !cli_option_seen {
        display_options = DisplayOptions::default_options();
    }

    // if we didn't find any file paths, read from stdin
    read_stdin = read_stdin || paths.is_empty();
    (display_options, paths, read_stdin)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_short_option() {
        assert_eq!(parse_short_option("-l"), DisplayOptions::with_lines_only());
        assert_eq!(parse_short_option("-w"), DisplayOptions::with_words_only());
        assert_eq!(parse_short_option("-m"), DisplayOptions::with_chars_only());
        assert_eq!(parse_short_option("-c"), DisplayOptions::with_bytes_only());
    }

    #[test]
    fn test_parse_long_option() {
        assert_eq!(
            parse_long_option("--lines"),
            DisplayOptions::with_lines_only()
        );
        assert_eq!(
            parse_long_option("--words"),
            DisplayOptions::with_words_only()
        );
        assert_eq!(
            parse_long_option("--chars"),
            DisplayOptions::with_chars_only()
        );
        assert_eq!(
            parse_long_option("--bytes"),
            DisplayOptions::with_bytes_only()
        );
    }

    #[test]
    fn test_parse_args() {
        let res = parse_args(&[String::from("--lines"), String::from("-m")]);
        assert_eq!(res.0, DisplayOptions::new(true, false, true, false));

        let res = parse_args(&[
            String::from("--lines"),
            String::from("-m"),
            String::from("-"),
            String::from("test"),
            String::from("--bytes"),
        ]);
        assert_eq!(res.0, DisplayOptions::new(true, false, true, true));
        assert_eq!(res.1, vec![PathBuf::from("test")]);
        assert_eq!(res.2, true);
        let res = parse_args(&[
            String::from("test"),
            String::from("a"),
            String::from("-"),
            String::from("1234"),
        ]);
        assert_eq!(res.0, DisplayOptions::default_options());
        assert_eq!(
            res.1,
            vec![
                PathBuf::from("test"),
                PathBuf::from("a"),
                PathBuf::from("1234")
            ]
        );
        assert_eq!(res.2, true);

        let res = parse_args(&[
            String::from("-c"),
            String::from("--chars"),
            String::from("-w"),
        ]);
        assert_eq!(res.0, DisplayOptions::new(false, true, true, true));
        assert!(res.1.is_empty());
        assert_eq!(res.2, true);
    }
}
