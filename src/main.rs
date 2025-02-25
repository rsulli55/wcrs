mod display_options;
mod file_result;

use display_options::DisplayOptions;
use file_result::{counts_for_line, file_result_string, FileResult};

use std::{env, io::Read};

const USAGE: &'static str = "wc [OPTION...] [FILE]...";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // TODO: read from stdin instead
        eprintln!("Error: expected a filename");
        eprintln!("Usage: {USAGE}");
        return;
    }

    let filename = &args[1];
    let mut file = match std::fs::File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Could not open file! {:?}", e);
            return;
        }
    };

    // TODO: what is faster?
    // let mut bytes = Vec::new();
    // file.read_to_end(&mut bytes);
    let mut contents = String::with_capacity(256);
    let num_bytes = match file.read_to_string(&mut contents) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Could not read file! {:?}", e);
            return;
        }
    };

    // TODO: Should we handle \r\n newlines as well?
    let result = contents
        .split_inclusive(|c| c == '\n')
        .fold(FileResult::default(), |acc, l| {
            let line_result = counts_for_line(l);
            FileResult::new(
                acc.bytes + line_result.bytes,
                acc.chars + line_result.chars,
                acc.lines + line_result.lines,
                acc.words + line_result.words,
            )
        });

    eprintln!("FileResult {:?}", result);

    println!(
        "{}",
        file_result_string(&result, &DisplayOptions::default())
    );
    println!("Read bytes: {}", num_bytes);
}

#[test]
fn test_file_result_string() {
    let result = FileResult::new(1, 2, 3, 4);
    let options = DisplayOptions::new(true, false, true, false);

    assert_eq!(
        &file_result_string(&result, &options),
        "1         3         "
    );
}
