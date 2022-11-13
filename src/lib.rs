use regex::Regex;
use std::error::Error;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .author("Myron Lioz <liozmyron@gmail.com")
        .about("Rust find")
        .version("0.1.0")
        .arg(
            Arg::with_name("name")
                .value_name("NAME")
                .help("Name")
                .short("n")
                .long("name")
                .multiple(true),
        )
        .arg(
            Arg::with_name("type")
                .value_name("TYPE")
                .help("Entry type")
                .short("t")
                .long("type")
                .multiple(true)
                .possible_values(&["d", "f", "l"]),
        )
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .help("Search paths")
                .multiple(true)
                .default_value("."),
        )
        .get_matches();

    Ok(Config {
        paths: matches.values_of_lossy("path").unwrap(),
        names: matches
            .values_of_lossy("name")
            .unwrap_or(Vec::new())
            .iter()
            .map(|pat| parse_regex(pat))
            .collect::<MyResult<Vec<Regex>>>()?,
        entry_types: matches.values_of_lossy("type").map_or(Vec::new(), |vc| {
            vc.iter()
                .map(|entry_type| match entry_type.as_str() {
                    "l" => EntryType::Link,
                    "f" => EntryType::File,
                    "d" => EntryType::Dir,
                    _ => unreachable!(),
                })
                .collect()
        }),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

fn parse_regex(pattern: &str) -> MyResult<Regex> {
    match Regex::new(pattern) {
        Err(_) => Err(From::from(format!("Invalid --name \"{}\"", pattern))),
        Ok(regex) => Ok(regex),
    }
}

#[cfg(test)]
mod test {
    use walkdir::WalkDir;
    #[test]
    fn test_walkdir() {
        let walk_dir = WalkDir::new("tests");
        for (i, entry) in walk_dir.into_iter().enumerate() {
            println!("{}: {}", i, entry.unwrap().path().display());
        }
    }
}
