fn gcd(a: u64, b: u64) -> Option<u64> {
    if a == 0 && b == 0 {
        return None; // GCD is undefined for both numbers being zero.
    }

    let mut x = a;
    let mut y = b;

    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }

    Some(x)
}

fn main() {
    let num1 = 48;
    let num2 = 18;

    match gcd(num1, num2) {
        Some(result) => {
            println!(
                "The greatest common divisor of {} and {} is: {}",
                num1, num2, result
            );
        }
        None => {
            println!("The GCD is undefined for two numbers being zero.");
        }
    }
}
