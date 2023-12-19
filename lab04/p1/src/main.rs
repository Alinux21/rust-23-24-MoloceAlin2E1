use std::{fs, io};

fn p1() -> Result<(), io::Error> {
    let s = fs::read_to_string("src/text.txt")?;
    println!("sunt un bou");
    for line in s.split("\n") {
        println!(
            "The line :'{}' contains {} characters and {} bytes.",
            line,
            line.chars().count(),
            line.as_bytes().len()
        );
    }
    
    Ok(())
}

fn main() {
    let _ = p1();
}
