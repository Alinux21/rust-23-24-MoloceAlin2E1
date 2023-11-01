use std::{fs, io};

fn p2() -> Result<String, io::Error> {
    let s: String = fs::read_to_string("src/p2.txt")?;

    for ch in s.chars() {
        if ch as u32 > 127 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "File contains non-ASCII character",
            ));
        }
    }

    let mut result = String::new();

    for index in s.bytes() {
        if (index >= 65 && index <= 77) || (index >= 97 && index <= 109) {
            result.push((index + 13) as char);
        } else if (index >= 78 && index <= 90) || (index >= 110 && index <= 122) {
            result.push((index - 13) as char);
        }
    }

    Ok(result)
}
fn main() {
    match p2() {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
