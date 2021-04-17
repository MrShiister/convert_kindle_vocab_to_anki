mod get_definition;
pub use crate::get_definition::get_definition;
use clap::ArgMatches;
use std::{
    fmt,
    process,
};

pub struct Word {
                // word: String, // original word
            headword: String, // 2.3 hw
      pronunciations: String, // 2.6 prs
    example_sentence: String, // imported
          definition: String, // 2.10.2 def
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // writeln!(f, "Word: {}\nHeadword: {}\nPronunciation: {}\nExample Sentence: {}\nDefinition: {}", self.word, self.headword, self.pronunciations, self.example_sentence, self.definition)
        writeln!(f, "Headword: {}\nPronunciation: {}\nExample Sentence: {}\nDefinition: {}", self.headword, self.pronunciations, self.example_sentence, self.definition)
    }
}

fn debug_print(message: String, message_verbosity: u8, verbosity: u8) {
    if verbosity >= message_verbosity {
        println!("{}", message);
    }
}

pub fn run(matches: ArgMatches) {

    // Get the desired verbosity
    let mut v = match matches.occurrences_of("v") {
        0 => 0,
        1 => 1,
        2 => 2,
        3 | _ => 3,
    };

    // Check if we are only doing test.
    if let Some(matches) = matches.subcommand_matches("test") {

        let mut vocablist: Vec<String> = Vec::new();

        let vocabargs = matches.values_of("vocab").unwrap();
        for vocab in vocabargs {
            vocablist.push(vocab.to_string());
        }

        // Minimum 1 verbosity for test
        if v == 0 { v = 1 };

        if let Err(e) = get_definition(vocablist, v) {
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
