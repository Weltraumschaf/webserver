use std::io::prelude::*;
use std::fs::File;

pub fn exists(file_name: &String) -> bool {
    match File::open(file_name) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn read_bytes(file_name: &String) -> Vec<u8> {
    debug!("Reading file {}.", file_name);
    let file = File::open(file_name)
        .expect("Can't open file {}!");
    let mut buffer: Vec<u8> = Vec::new();

    for byte in file.bytes() {
        buffer.push(byte.expect("Can't read byte from file!"));
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn test_exists() {
        assert_that!(exists(&String::from("test_fixture.txt")), is(true));
        assert_that!(exists(&String::from("snafu")), is(false));
    }

    #[test]
    fn test_read_bytes() {
        let content = read_bytes(&String::from("test_fixture.txt"));

        assert_that!(
            content,
            is(equal_to(vec!(72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33)))
        );
    }
}