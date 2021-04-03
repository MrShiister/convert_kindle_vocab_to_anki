mod get_definition;
pub use crate::get_definition::get_definition;
use clap::ArgMatches;
use std::process;

fn debug_print(message: String, message_verbosity: u8, verbosity: u8) {
    if verbosity >= message_verbosity {
        println!("{}", message);
    }
}

pub fn run(matches: ArgMatches) {

    // Get the desired verbosity
    let v = match matches.occurrences_of("v") {
        0 => 0,
        1 => 1,
        2 | _ => 2,
    };

    // Check if we are only doing test.
    if let Some(matches) = matches.subcommand_matches("test") {
        let vocab = matches.value_of("vocab").unwrap();
        debug_print(format!("Testing on vocab: {}", vocab), 0, v);
        if let Err(e) = get_definition(&vocab, v) {
            eprintln!("Failed to get definition: {}", e);
            process::exit(1);
        }
    } else {
    // We are doing the real thing.
        debug_print(format!("Writing results to: {}", matches.value_of("OUTFILE").unwrap()), 1, v);
        debug_print(format!("Path to words.csv: {}", matches.value_of("words").unwrap()), 1, v);
        debug_print(format!("Path to lookups.csv: {}", matches.value_of("lookups").unwrap()), 1, v);
        debug_print(format!("Verbosity: {}", v), 0, v);
    }
}
