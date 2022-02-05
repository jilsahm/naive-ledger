use std::{path::PathBuf, fs::File};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Configuration {
    #[clap(
        help = "the source CSV file to be parsed",
        validator = valid_path,
        env = "NAIVE_PARSER_SOURCE"
    )]
    pub source: PathBuf,
}

fn valid_path(s: &str) -> Result<(), String> {
    File::open(s)
        .map(|_| ())
        .map_err(|_| format!("'{}' is not a readable file", s))
}

#[cfg(test)]
mod tests {

    #[test]
    fn valid_path() {
        let result = super::valid_path("./Cargo.toml");
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_path() {
        let result = super::valid_path("./src");
        assert!(result.is_err());
    }
}