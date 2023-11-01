use std::{fs, io};

fn p3() -> Result<(), io::Error> {
    let sentence = fs::read_to_string("src/p3.txt")?;
    let mut result: String = String::new();

    for word in sentence.split(" ") {
        if word == "pt" {
            result.push_str("pentru");
            result.push(' ');
        } else if word == "ptr" {
            {
                result.push_str("pentru");
                result.push(' ');
            }
        } else if word == "dl" {
            {
                result.push_str("domnul");
                result.push(' ');
            }
        } else if word == "dna" {
            {
                result.push_str("doamna");
                result.push(' ');
            }
        } else {
            result.push_str(word);
            result.push(' ');
        }
    }
    println!("{}", result);

    Ok(())
}

fn main() {
    let _ = p3();
}
