use std::{env, io};

fn get_env() -> Vec<String> {
    let vars: Vec<String> = env::args()
        .map(|x| x.trim().to_string())
        .collect();
    vars
}

fn arg_handler(arg: &str) {
    todo!();
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::get_env;

    #[test]
    fn test_get_env() {
        assert_eq!(vec!(std::env::current_exe().unwrap().into_os_string().into_string().unwrap()), get_env());
    }
}
