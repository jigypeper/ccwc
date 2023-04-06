use std::{env, fs};

#[derive(Debug, PartialEq)]
struct Count {
    bytes: usize,
    characters: usize,
    words: usize,
    lines: usize
}

fn get_env() -> Vec<String> {
    let vars: Vec<String> = env::args()
        .map(|x| x.trim().to_string())
        .collect();
    vars
}

fn arg_handler(arg: &str) {
    todo!();
}

fn get_stats(file_path: &str) -> Count {
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

fn main() {
    todo!();
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
