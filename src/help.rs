use crate::VERSION;

pub fn usage_text() -> String {
    "\
Usage: bsort [OPTIONS] [FILE]

Options:
  --desc          Sort in descending order
  --unique        Remove duplicate values after sorting
  --count         Print the parsed integer count
  --check         Check whether input is already sorted
  --ignore-blank  Ignore blank lines
  --help          Print usage and exit 0
  --version       Print version and exit 0
"
    .to_string()
}

pub fn version_text() -> String {
    format!("bsort {VERSION}\n")
}
