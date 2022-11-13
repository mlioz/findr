use regex::Regex;
use std::{error::Error, fs::FileType};

use clap::{App, Arg};
use walkdir::WalkDir;

use crate::EntryType::*; // Allows 'Dir' instead of 'EntryType::Dir'

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
        .author("Myron Lioz <liozmyron@gmail.com>")
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
        entry_types: matches
            .values_of_lossy("type")
            .map_or(vec![Dir, File, Link], |vc| {
                vc.iter()
                    .map(|entry_type| match entry_type.as_str() {
                        "l" => EntryType::Link,
                        "f" => EntryType::File,
                        "d" => EntryType::Dir,
                        _ => unreachable!("Invalid file type"),
                    })
                    .collect()
            }),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for path in &config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(path) => {
                    let filename = path
                        .file_name()
                        .to_str()
                        .ok_or::<Box<dyn Error>>(From::from("Failed to parse path url unicode"))?;

                    let is_name = config.names.len() == 0 || is_correct_name(&filename, &config);
                    let is_type = config.entry_types.len() == 3
                        || is_correct_type(&path.file_type(), &config);

                    if is_name && is_type {
                        println!("{}", path.path().display())
                    }
                }
            }
        }
    }

    Ok(())
}

fn parse_regex(pattern: &str) -> MyResult<Regex> {
    match Regex::new(pattern) {
        Err(_) => Err(From::from(format!("Invalid --name \"{}\"", pattern))),
        Ok(regex) => Ok(regex),
    }
}

fn is_correct_name(filename: &str, config: &Config) -> bool {
    config.names.iter().any(|pat| pat.is_match(filename))
}

fn is_correct_type(file_type: &FileType, config: &Config) -> bool {
    if file_type.is_dir() && config.entry_types.contains(&Dir) {
        return true;
    } else if file_type.is_file() && config.entry_types.contains(&File) {
        return true;
    } else if file_type.is_symlink() && config.entry_types.contains(&Link) {
        return true;
    }

    false
}
