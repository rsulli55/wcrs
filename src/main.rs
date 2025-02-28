use std::process::ExitCode;
use std::{env, io::Read};
use wcrs::cli_args::parse_args;
use wcrs::constants::{EXIT_FAILURE, EXIT_SUCCESS, PROGRAM, USAGE};
use wcrs::file_result::{counts_for_file, file_result_string, FileResult};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // TODO: read from stdin instead
        eprintln!("Error: expected a filename");
        eprintln!("Usage: {USAGE}");
        return ExitCode::from(EXIT_FAILURE);
    }

    let (display_options, paths, read_stdin) = parse_args(&args[1..]);

    let mut return_exit_failure = false;
    let mut total = FileResult::default();
    let print_total = paths.len() > 1 || (paths.len() == 1 && read_stdin);
    for path in paths {
        let mut file = match std::fs::OpenOptions::new().read(true).open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{PROGRAM}: {}: {}", &path.to_string_lossy(), &e);
                return_exit_failure = true;
                continue;
            }
        };

        // TODO: what is faster?
        // let mut bytes = Vec::new();
        // file.read_to_end(&mut bytes);
        let mut contents = String::with_capacity(256);
        let num_bytes = match file.read_to_string(&mut contents) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("{PROGRAM}: {}: {}", &path.to_string_lossy(), &e);
                return_exit_failure = true;
                continue;
            }
        };

        // compute the counts for the file and accumulate in total
        let result = counts_for_file(&contents);
        total.add_mut(&result);

        println!(
            " {}  {}",
            file_result_string(&result, &display_options),
            &path.to_string_lossy()
        );
    }

    if print_total {
        println!(" {}  total", file_result_string(&total, &display_options),);
    }

    if return_exit_failure {
        ExitCode::from(EXIT_FAILURE)
    } else {
        ExitCode::from(EXIT_SUCCESS)
    }
}
