fn main() {
    if let Err(message) = bsort::run() {
        eprintln!("{message}");
        std::process::exit(1);
    }
}
