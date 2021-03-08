mod arg_parse;
pub use crate::arg_parse::arg_parse;

pub struct Config {
    outfilename: Option<String>,
    test: Option<bool>,
    words_csv: String,
    lookups_csv: String,
}

pub fn run(config: Config) {
    arg_parse(config);
}
