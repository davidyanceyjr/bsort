fn main() {
    if let Err(err) = bsort::run() {
        eprintln!("{}", err.message);
        std::process::exit(err.exit_code);
    }
}
