use clap::{Arg, App, SubCommand};
use convert_kindle_vocab_to_anki::run;
use std::time::Instant;

fn main() {

    let start_time = Instant::now();

    // argparse
    let matches = App::new("Convert Kindle Vocab to Anki")
                          .version("1.0")
                          .author("https://github.com/MrShiister")
                          .about("Collects the vocabulary words from your Kindle and convert them into an Anki-importable format.")
                          .arg(Arg::with_name("OUTFILE")
                               .short("o")
                               .long("outfile")
                               .value_name("FILE")
                               .default_value("./to_import.tsv")
                               .help("Sets the path of the tsv output")
                               .takes_value(true))
                          .arg(Arg::with_name("vocabdb")
                               .short("d")
                               .long("vocabdb")
                               .default_value("./vocab.db")
                               .value_name("path/to/vocab.db")
                               .help("Specify the path of vocab.db")
                               .takes_value(true))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .arg(Arg::with_name("timestamp")
                               .short("t")
                               .long("from")
                               .default_value("0")
                               .value_name("epoch_timestamp")
                               .help("Specify the timestamp as in the database file to start collecting words from. Useful if you do not want to import all words - only words queried in your Kindle from a certain time. Use epoch timestamp, e.g. 1571009240989"))
                          .subcommand(SubCommand::with_name("test")
                                      .about("Specify word(s) to return definitions of, without writing to file.")
                                      .arg(Arg::with_name("words")
                                          .default_value("beautiful")
                                          //.required(true)
                                          .takes_value(true)
                                          .min_values(1)
                                          .help("Vocab to test")
                                          .index(1)))
                          .get_matches();

    if let Err(e) = run(matches) {
        eprintln!("Failed to run run(matches): {}", e);
    }

    let duration = start_time.elapsed();
    println!("Time elapsed: {:?}", duration);

}
