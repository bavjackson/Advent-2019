use std::{fs, path::Path};

pub fn read_input(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|e| panic!("could not read {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_file_contents() {
        assert_eq!("this is a test\n", read_input("./test.txt"))
    }
}
