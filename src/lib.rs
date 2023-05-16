use std::{fs::{self, File}, path::PathBuf, io::{BufRead, BufReader, stdin}};
use clap::{Parser};

#[derive(Parser)]
pub struct Cli {

    /// file path
    pub file: PathBuf,

    /// number of bytes
    #[arg(default_value_t= String::from("all"))]
    pub count: String,
}

pub fn arg_handler(args: Cli) {
    let file = args.file;

    if file != PathBuf::from("-") {
        if args.count == String::from("all") {

            let data = get_stats(BufReader::new(File::open(&file).unwrap()));
    
            println!("{} {} {} {} {}", data.bytes, data.characters, data.words, data.lines, file.as_path().as_os_str().to_string_lossy());
    
        } else {
    
            let data = get_stats(BufReader::new(File::open(&file).unwrap())); 
    
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
    } else {
        let file = PathBuf::from("<stdin>");
        if args.count == String::from("all") {

            let data = get_stats(BufReader::new(stdin().lock()));
    
            println!("{} {} {} {} {}", data.bytes, data.characters, data.words, data.lines, file.as_path().as_os_str().to_string_lossy());
    
        } else {
    
            let data = get_stats(BufReader::new(stdin().lock())); 
    
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

    
    
}

#[derive(Debug, PartialEq)]
pub struct Count {
    pub bytes: usize,
    pub characters: usize,
    pub words: usize,
    pub lines: usize
}

pub fn get_stats<R: BufRead>(buf_reader: R) -> Count {
    let mut words = 0;
    let mut bytes = 0;
    let mut characters = 0;
    let mut lines = 0;

    for line in buf_reader.lines() {
        let working_line = line.unwrap().clone();
        words += working_line.clone().split(' ').count();
        lines += 1;
        bytes += working_line.clone().len();
        characters += working_line.clone().chars().count();
    }

    let output = Count {
        bytes: bytes,
        characters: characters,
        words: words,
        lines: lines,
    };

    // match data {
    //     Ok(contents) => {
    //         let ref_contents = &contents;
    //         output.bytes = ref_contents.len();
    //         output.characters = ref_contents.replace("\n", "").chars().count();
    //         output.words = ref_contents.split_whitespace().collect::<Vec<&str>>().len();
    //         output.lines = ref_contents.lines().collect::<Vec<&str>>().len();

    //     },
    //     Err(_) => eprintln!("Invalid file path")
    // }

    output
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::*;

    const ARGUMENT: &str = "all";
    const CORRECT_PATH: &str = "./README.md";
    const FAKE_PATH: &str = "./some_non_existent_file.md";

    #[test]
    fn test_count_valid() {
        let file: Cli = Cli {
            count: String::from(ARGUMENT),
            file: PathBuf::from(CORRECT_PATH)
        };

        assert_eq!(Count {
            bytes: 18,
            characters: 18,
            words: 5,
            lines: 2

        },
    get_stats(BufReader::new(File::open(&file.file).unwrap())));
    }

    #[test]
    #[should_panic]
    fn test_count_invalid() {
        let fake_file: Cli = Cli {
            count: String::from(ARGUMENT),
            file: PathBuf::from(FAKE_PATH)
        };
        
        assert_eq!(Count {
            bytes: 0,
            characters: 0,
            words: 0,
            lines: 0

        },
    get_stats(BufReader::new(File::open(&fake_file.file).expect("File does not exist"))));
    }
}