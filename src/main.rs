use std::{path::PathBuf, io};

use ccwc::{
    arg_handler,
    Cli,
};

use clap::Parser;

fn main() {
    let args = Cli::parse();

    let mut file = args.file.clone();

    if file == PathBuf::from("-") {

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        file = PathBuf::from(buffer);
        let func_arg = Cli {
            file: file.clone(),
            ..args
        };
        // println!("{}", func_arg.file.as_path().as_os_str().to_string_lossy());
        arg_handler(func_arg);
    } else {
        arg_handler(args);
    }
}
