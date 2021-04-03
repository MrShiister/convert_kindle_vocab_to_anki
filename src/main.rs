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
                          .arg(Arg::with_name("words")
                               .short("w")
                               .long("words")
                               .default_value("words.tsv")
                               .value_name("path/to/words.csv")
                               .help("Specify the path of words.csv")
                               .takes_value(true))
                               //.required(true)
                               //.index(1))
                          .arg(Arg::with_name("lookups")
                               .short("l")
                               .long("lookups")
                               .default_value("lookups.tsv")
                               .value_name("path/to/lookups.csv")
                               .help("Specify the path of lookups.csv")
                               .takes_value(true))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("0.1")
                                      .arg(Arg::with_name("vocab")
                                          .short("b")
                                          .default_value("beautiful")
                                          .takes_value(true)
                                          .help("Vocab to test")
                                          .index(1)))
                          .get_matches();

    run(matches);

    let duration = start_time.elapsed();
    println!("Time elapsed: {:?}", duration);

}
