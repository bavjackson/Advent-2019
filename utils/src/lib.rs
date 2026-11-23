use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_file_contents() {
        assert_eq!("this is a test\n", read_input("./test.txt"))
    }
}
