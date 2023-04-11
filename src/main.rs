use std::{path::PathBuf, io::BufReader, fs::File};

use ccwc::{
    get_env,
    arg_handler,
};

use is_terminal::IsTerminal as _;


fn main() {
    let args = get_env();
    let _stats = arg_handler(args);

    let word_count;
    let mut file = args.file;

    if file == PathBuf::from("-") {

        file = PathBuf::from("<stdin>");
        word_count = words_in_buf_reader(BufReader::new(stdin().lock()));
    } else {
        word_count = words_in_buf_reader(BufReader::new(File::open(&file).unwrap()));
    }

    println!("Words from {}: {}", file.to_string_lossy(), word_count)
}
