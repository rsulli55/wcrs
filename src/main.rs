use std::{env, io::Read};
use wcrs::display_options::DisplayOptions;
use wcrs::file_result::{counts_for_file, file_result_string};

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

    let result = counts_for_file(&contents);

    println!(
        "{}  {}",
        file_result_string(&result, &DisplayOptions::default()),
        &filename
    );
    println!("Read bytes: {}", num_bytes);
}
