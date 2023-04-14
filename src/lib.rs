use std::{io::{stdin, BufRead}, env, fs};
use clap::{Parser, Args};

#[derive(Parser)]
struct Cli {
    /// number of bytes
    #[default_value_t = CountOptions::all]
    count: CountOptions,

    /// file path
    #[arg(short)]
    file: PathBuf,
}

enum CountOptions {
    /// word count
    #[args(short)]
    words,

    /// byte count
    #[args(short)]
    bytes,

    /// character count
    #[args(short)]
    characters,

    /// line count
    #[args(short)]
    lines,

    /// all results
    #[args(short)]
    all,
}

pub fn arg_handler(args: Vec<String>) {
    let mut file = String::from("");

    if args.len() > 1 {
        file = args.iter()
            .next().expect("File does not exist").to_string();

        let data = get_stats(&file);

        println!("{} {} {} {} {}", data.bytes, data.characters, data.words, data.lines, file);

    } else if args.len() > 2 {
        file = args.iter()
            .next().expect("File does not exist").to_string();

        let data = get_stats(&file); 

        let data_type = &args.iter()
            .next().unwrap()[..];

        match data_type {
            "-c" => println!("{} {}", data.characters, file),
            "-l" => println!("{} {}", data.lines, file),
            "-w" => println!("{} {}", data.words, file),
            "-b" => println!("{} {}", data.bytes, file),
            _ => println!("Not a valid option")
        }

    } else {
        let data = get_stats(&stdin().lock().lines().next().unwrap().unwrap()[..]);
        println!("{} {} {} {} {}", data.bytes, data.characters, data.words, data.lines, file);
    }
    
}

#[derive(Debug, PartialEq)]
pub struct Count {
    pub bytes: usize,
    pub characters: usize,
    pub words: usize,
    pub lines: usize
}

pub fn get_env() -> Vec<String> {
    let vars: Vec<String> = env::args()
        .map(|x| x.trim().to_string())
        .collect();
    vars
}

pub fn get_stats(file_path: &str) -> Count {
    let data = fs::read_to_string(file_path);
    let mut output = Count {
        bytes: 0,
        characters: 0,
        words: 0,
        lines: 0,
    };

    match data {
        Ok(contents) => {
            let ref_contents = &contents;
            output.bytes = ref_contents.len();
            output.characters = ref_contents.replace("\n", "").chars().count();
            output.words = ref_contents.split_whitespace().collect::<Vec<&str>>().len();
            output.lines = ref_contents.lines().collect::<Vec<&str>>().len();

        },
        Err(_) => eprintln!("Invalid file path")
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_env() {
        assert_eq!(vec!(std::env::current_exe().unwrap().into_os_string().into_string().unwrap()), get_env());
    }

    #[test]
    fn test_count_valid() {
        assert_eq!(Count {
            bytes: 20,
            characters: 19,
            words: 5,
            lines: 2

        },
    get_stats("./README.md"));
    }

    #[test]
    fn test_count_invalid() {
        assert_eq!(Count {
            bytes: 0,
            characters: 0,
            words: 0,
            lines: 0

        },
    get_stats("./some_non_existent_file.md"));
    }
}