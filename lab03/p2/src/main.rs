use std::mem;

fn addition(x: u32, y: u32) -> u32 {
    let sum = x + y;
    if mem::size_of_val(&sum) > std::u32::MAX.try_into().unwrap() {
        panic!("The sum of {} and {} does not fit in u32", x, y);
    }
    return sum;
}

fn multiplication(x: u32, y: u32) -> u32 {
    let product = x * y;
    if mem::size_of_val(&product) > std::u32::MAX.try_into().unwrap() {
        panic!("The product of {} and {} does not fit in u32", x, y);
    }
    return product;
}

fn main() {
    let r = std::u32::MAX;
    println!("{}", addition(r, 0));
    println!("{}", multiplication(r, 2));
}
