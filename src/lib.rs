use std::{io::{stdin, BufRead}, env, fs, path::PathBuf};
use clap::{Parser, Args};

#[derive(Parser)]
pub struct Cli {
    /// number of bytes
    #[arg(default_value_t= String::from("all"))]
    pub count: String,

    /// file path
    #[arg(short)]
    pub file: PathBuf,
}

pub fn arg_handler(args: Cli) {
    let mut file = args.file;

    if args.count == String::from("all") {
        file = args.file;

        let data = get_stats(args);

        println!("{} {} {} {} {}", data.bytes, data.characters, data.words, data.lines, file.as_path().as_os_str().to_string_lossy());

    } else {

        let data = get_stats(args); 

        let data_type = args.count;

        match &data_type[..] {
            "-c" => println!("{} {}", data.characters, file.as_path().as_os_str().to_string_lossy()),
            "-l" => println!("{} {}", data.lines, file.as_path().as_os_str().to_string_lossy()),
            "-w" => println!("{} {}", data.words, file.as_path().as_os_str().to_string_lossy()),
            "-b" => println!("{} {}", data.bytes, file.as_path().as_os_str().to_string_lossy()),
            "all" => println!("{} {} {} {} {}", data.bytes, data.characters, data.words, data.lines, file.as_path().as_os_str().to_string_lossy()),
            _ => println!("invalid option")
        }

    }
    
}

#[derive(Debug, PartialEq)]
pub struct Count {
    pub bytes: usize,
    pub characters: usize,
    pub words: usize,
    pub lines: usize
}

pub fn get_stats(file_path: Cli) -> Count {
    let data = fs::read_to_string(file_path.file);
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
    use std::path::PathBuf;

    use crate::*;

    const file: Cli = Cli {
        count: String::from("all"),
        file: PathBuf::from("./README.md")
    };

    const fake_file: Cli = Cli {
        count: String::from("all"),
        file: PathBuf::from("./some_non_existent_file.md")
    };

    #[test]
    fn test_count_valid() {
        assert_eq!(Count {
            bytes: 20,
            characters: 19,
            words: 5,
            lines: 2

        },
    get_stats(file));
    }

    #[test]
    fn test_count_invalid() {
        assert_eq!(Count {
            bytes: 0,
            characters: 0,
            words: 0,
            lines: 0

        },
    get_stats(fake_file));
    }
}