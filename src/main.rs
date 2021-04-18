use clap::{Arg, App, SubCommand};
use convert_kindle_vocab_to_anki::{run};
use std::time::Instant;

fn main() {

    let start_time = Instant::now();

    // argparse
    let matches = App::new("Convert Kindle Vocab to Anki")
                          .version("0.1")
                          .author("https://github.com/MrShiister")
                          .about("Collects the vocabulary words from your Kindle and convert them into an Anki-importable format.")
                          .arg(Arg::with_name("OUTFILE")
                               .short("o")
                               .long("outfile")
                               .value_name("FILE")
                               .default_value("myvocab.tsv")
                               .help("Sets the path of the tsv output")
                               .takes_value(true))
                          .arg(Arg::with_name("vocabdb")
                               .short("d")
                               .long("vocabdb")
                               .default_value("vocab.db")
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
                               .help("Specify the time to start collecting words from. Use epoch timestamp, e.g. 1571009240989"))
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("0.1")
                                      .arg(Arg::with_name("words")
                                          .default_value("beautiful")
                                          //.required(true)
                                          .takes_value(true)
                                          .min_values(1)
                                          .help("Vocab to test")
                                          .index(1)))
                          .get_matches();

    run(matches);

    let duration = start_time.elapsed();
    println!("Time elapsed: {:?}", duration);

}
