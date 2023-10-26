use std::mem;

fn is_prime(x: u16) -> bool {
    if x <= 1 {
        return false;
    }
    for i in 2..x {
        if x % i == 0 {
            return false;
        }
    }
    true
}

fn next_prime(x: u16) -> Option<u16> {
    let mut prime = x + 1;
    while !is_prime(prime) {
        prime += 1;
    }
    if mem::size_of_val(&prime) >= std::u16::MAX.into() {
        return None;
    }

    Some(prime)
}

fn main() {
    let mut starting_number: u16 = 1;
    let mut r = next_prime(starting_number);

    while (r.is_some()) {
        if r.is_some() {
            println!("Next prime is:{}", r.unwrap());
        } else {
            println!("Next prime does not fit in u16");
        }
        starting_number = r.unwrap();
        r = next_prime(starting_number);
    }
}
