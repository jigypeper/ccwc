use std::{path::PathBuf, io};

use ccwc::{
    arg_handler,
    Cli,
};

use clap::Parser;

fn main() {
    let args = Cli::parse();

    arg_handler(args);

    // let mut file = args.file.clone();

    // if file == PathBuf::from("-") {

    //     let mut buffer = String::new();
    //     io::stdin().read_line(&mut buffer).unwrap();

    //     file = PathBuf::from(buffer);
    //     let func_arg = Cli {
    //         file: file.clone(),
    //         ..args
    //     };
    //     // println!("{}", func_arg.file.as_path().as_os_str().to_string_lossy());
    //     arg_handler(func_arg);
    // } else {
    //     arg_handler(args);
    // }
}

// #[cfg(test)]
// mod tests {

//     use std::{io::BufReader, fs::File};

//     use ccwc::{get_stats, Count};

//     use super::*;

//     #[test]
//     fn test_stdin() {
//         let arg = Cli {
//             file: PathBuf::from("-"),
//             count: String::from("-w")
//         };

//         let mut buffer = String::new();
//         io::stdin().read_line(&mut buffer).unwrap().to_string();

//         let file = PathBuf::from(buffer);
//         let func_arg = Cli {
//             file: file.clone(),
//             ..arg
//         };

//         assert_eq!(Count {
//             bytes: 19,
//             characters: 18,
//             words: 5,
//             lines: 2

//         },
//     get_stats(BufReader::new(File::open(&func_arg.file).unwrap())));
//     }

// }