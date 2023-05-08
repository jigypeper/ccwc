use std::{path::PathBuf};

use ccwc::{
    arg_handler,
    Cli,
};

use clap::Parser;
// use is_terminal::IsTerminal as _;


fn main() {
    let args = Cli::parse();
    // let _stats = arg_handler(args);

    let mut file = args.file.clone();

    if file == PathBuf::from("-") {

        file = PathBuf::from("<stdin>");
        let func_arg = Cli {
            file: file,
            ..args
        };
        arg_handler(func_arg);
    } else {
        arg_handler(args);
    }
}
