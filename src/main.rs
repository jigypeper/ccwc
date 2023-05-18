use ccwc::{
    arg_handler,
    Cli,
};

use clap::Parser;

fn main() {
    let args = Cli::parse();

    arg_handler(args);

}
