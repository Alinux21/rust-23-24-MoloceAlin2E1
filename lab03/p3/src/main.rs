use std::mem;

enum MyError {
    Addition,
    Multiplication,
}

fn addition(x: u32, y: u32) -> Result<u32, MyError> {
    let sum = x + y;
    if mem::size_of_val(&sum) > std::u32::MAX.try_into().unwrap() {
        Err(MyError::Addition)
    } else {
        Ok(sum)
    }
}

fn multiplication(x: u32, y: u32) -> Result<u32, MyError> {
    let product = x * y;
    if mem::size_of_val(&product) > std::u32::MAX.try_into().unwrap() {
        Err(MyError::Multiplication)
    } else {
        Ok(product)
    }
}

fn perform_operations(x: u32, y: u32) -> Result<u32, MyError> {
    let sum = addition(x, y)?;
    let product = multiplication(sum, y)?;
    Ok(product)
}

fn main() {
    let x = 10;
    let y = 20;

    match addition(x, y) {
        Ok(result) => {
            println!("Addition result: {}", result);
        }
        Err(MyError::Addition) => {
            println!("Error: Overflow in addition");
        }
        Err(_) => {
            println!("Unknown error occurred in addition");
        }
    }

    match multiplication(x, y) {
        Ok(result) => {
            println!("Multiplication result: {}", result);
        }
        Err(MyError::Multiplication) => {
            println!("Error: Overflow in multiplication");
        }
        Err(_) => {
            println!("Unknown error occurred in multiplication");
        }
    }

    match perform_operations(x, y) {
        Ok(result) => {
            println!("Final result: {}", result);
        }
        Err(err) => match err {
            MyError::Addition => {
                println!("Error: Overflow in addition");
            }
            MyError::Multiplication => {
                println!("Error: Overflow in multiplication");
            }
        },
    }
}
