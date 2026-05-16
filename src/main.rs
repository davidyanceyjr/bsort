fn main() {
    if let Err(err) = bsort::run() {
        if !err.message.is_empty() {
            eprintln!("{}", err.message);
        }
        std::process::exit(err.exit_code);
    }
}
