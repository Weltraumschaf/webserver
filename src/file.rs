use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

pub fn exists(file_name: &String) -> bool {
    match File::open(file_name) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn read_bytes(file_name: &PathBuf) -> Vec<u8> {
    debug!("Reading file {:?}.", file_name);
    let file = File::open(file_name)
        .expect("Can't open file {}!");
    let mut buffer: Vec<u8> = Vec::new();

    for byte in file.bytes() {
        buffer.push(byte.expect("Can't read byte from file!"));
    }

    buffer
}

pub fn read_string(file_name: &String) -> String {
    debug!("Reading file {}.", file_name);
    let mut file = File::open(file_name)
        .expect("Can't open file {}!");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn test_exists() {
        assert_that!(exists(&String::from("test/hello.txt")), is(true));
        assert_that!(exists(&String::from("snafu")), is(false));
    }

    #[test]
    fn test_read_bytes() {
        let content = read_bytes(&PathBuf::from("test/hello.txt"));

        assert_that!(
            content,
            is(equal_to(vec!(72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33)))
        );
    }

    #[test]
    fn test_read_string() {
        let content = read_string(&String::from("test/hello.txt"));

        assert_that!(
            content,
            is(equal_to(String::from("Hello, World!")))
        );
    }
}