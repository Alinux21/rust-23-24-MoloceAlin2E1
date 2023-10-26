enum Errors {
    NotASCII,
    NotDigit,
    Notbase16,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(x: char) -> Result<char, Errors> {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lowercase = "abcdefghijklmnopqrstuvwxyz";

    if !letters.contains(x) {
        return Err(Errors::NotLetter);
    }

    let result = if lowercase.contains(x) {
        x.to_uppercase().next().unwrap()
    } else {
        x
    };

    Ok(result)
}

fn to_lowercase(x: char) -> Result<char, Errors> {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    if !letters.contains(x) {
        return Err(Errors::NotLetter);
    }

    let result = if uppercase.contains(x) {
        x.to_lowercase().next().unwrap()
    } else {
        x
    };

    Ok(result)
}

fn print_char(c: char) -> Result<(), Errors> {
    if !c.is_ascii() {
        return Err(Errors::NotASCII);
    }
    if !c.is_ascii_digit() {
        return Err(Errors::NotDigit);
    }
    if !c.is_ascii_hexdigit() {
        return Err(Errors::NotBase16);
    }
    if !c.is_alphabetic() {
        return Err(Errors::NotLetter);
    }
    if !c.is_ascii_graphic() {
        return Err(Errors::NotPrintable);
    }
    Ok(())
}

fn char_to_number(c: char) -> Result<u32, Errors> {
    print_char(c)?;

    if c.is_ascii_digit() {
        Ok(c.to_digit(10).unwrap())
    } else {
        Err(Errors::NotDigit)
    }
}

fn char_to_number_hex(c: char) -> Result<u32, Errors> {
    print_char(c)?;

    if c.is_ascii_hexdigit() {
        Ok(c.to_digit(16).unwrap())
    } else {
        Err(Errors::NotBase16)
    }
}

fn print_error(error: Errors) {
    match error {
        Errors::NotDigit => {
            println!("Error: Not a digit.");
        }
        Errors::NotBase16 => {
            println!("Error: Not a base-16 digit.");
        }
    }
}

fn main() {
    let x: char = 'a';
    let y: char = 'b';
    let v = to_uppercase(x);
    let w = to_lowercase(y);

    let digit_char: char = '5';
    let hex_char: char = 'A';
    let non_digit_char: char = 'X';

    match v {
        Ok(result) => {
            println!("Letter is now {}", result);
        }
        Err(Errors::NotLetter) => {
            println!("{} is not a letter", x);
        }
    }

    match w {
        Ok(result) => {
            println!("Letter is now {}", result);
        }
        Err(Errors::NotLetter) => {
            println!("{} is not a letter", y);
        }
    }

    match char_to_number(digit_char) {
        Ok(result) => {
            println!("Character to number (base 10): {}", result);
        }
        Err(error) => {
            print_error(error);
        }
    }

    match char_to_number_hex(hex_char) {
        Ok(result) => {
            println!("Character to number (hex): {}", result);
        }
        Err(error) => {
            print_error(error);
        }
    }

    match char_to_number(non_digit_char) {
        Ok(result) => {
            println!("Character to number (base 10): {}", result);
        }
        Err(error) => {
            print_error(error);
        }
    }
}
