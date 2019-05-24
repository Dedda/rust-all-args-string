use std::env;

pub fn all_args_string() -> String {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    args.join(" ")
}
