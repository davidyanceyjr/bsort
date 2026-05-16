use std::io::{self, Read};

pub fn read_stdin_text() -> io::Result<String> {
    read_all(std::io::stdin())
}

fn read_all<R: Read>(mut reader: R) -> io::Result<String> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::read_all;

    #[test]
    fn read_all_handles_empty_input() {
        assert_eq!(read_all("".as_bytes()).expect("read should succeed"), "");
    }

    #[test]
    fn read_all_handles_text_input() {
        assert_eq!(
            read_all("1\n2\n3\n".as_bytes()).expect("read should succeed"),
            "1\n2\n3\n"
        );
    }
}
