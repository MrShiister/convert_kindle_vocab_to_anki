mod get_definition;
mod get_lookups;
pub use crate::get_definition::get_definition;
pub use crate::get_lookups::get_lookups;
use clap::ArgMatches;
use std::{
    fmt,
    process,
};

pub struct Word {
            word_key: String, // word key as recorded by Kindle
                word: String, // original word
            headword: String, // 2.3 hw
      pronunciations: String, // 2.6 prs
    example_sentence: String, // imported
          definition: String, // 2.10.2 def
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Word Key: {}\nWord: {}\nHeadword: {}\nPronunciation: {}\nExample Sentence: {}\nDefinition: {}", self.word_key, self.word, self.headword, self.pronunciations, self.example_sentence, self.definition)
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

    let mut wordlist: Vec<Word> = Vec::new();

    // Check if we are only doing test.
    if let Some(matches) = matches.subcommand_matches("test") {

        let words = matches.values_of("words").unwrap();
        for vocab in words {
            let word = Word {
                        word_key: "".to_string(), // original word
                            word: vocab.to_string(), // original word
                        headword: "".to_string(), // 2.3 hw
                  pronunciations: "".to_string(), // 2.6 prs
                example_sentence: "".to_string(), // imported
                      definition: "".to_string(), // 2.10.2 def
            };

            wordlist.push(word);
        }

        // Minimum 1 verbosity for test
        if v == 0 { v = 1 };

        if let Err(e) = get_definition(&mut wordlist, v) {
            eprintln!("Failed to get definition: {}", e);
            process::exit(1);
        }

    } else {
    // We are doing the real thing.
        debug_print(format!("Writing results to: {}", matches.value_of("OUTFILE").unwrap()), 1, v);
        debug_print(format!("Path to vocab.db: {}", matches.value_of("vocabdb").unwrap()), 1, v);
        debug_print(format!("Verbosity: {}", v), 0, v);

        // read the vocab.db database
        let vocabdb_path = matches.value_of("vocabdb").unwrap().to_string();
        let timestamp = matches.value_of("timestamp").unwrap().parse::<u64>().unwrap();

        wordlist = match get_lookups(vocabdb_path, timestamp, v) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to get lookups from database: {}", e);
                process::exit(1);
            }
        };

        // add words and example sentences to Word struct
        if let Err(e) = get_definition(&mut wordlist, v) {
            eprintln!("Failed to get definition: {}", e);
            process::exit(1);
        }

        // TODO: Write them to the final tsv
    }


}
