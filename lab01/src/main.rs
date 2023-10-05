fn isprime(x: i32) -> bool {
    let mut prime = true;
    for i in 2..=x / 2 {
        if x % i == 0 {
            prime = false;
            break;
        }
    }
    return prime;
}

fn coprimes(x: i32, y: i32) -> bool {
    let mut coprime = true;

    for i in 2..=x {
        if x % i == 0 && y % i == 0 {
            coprime = false;
            break;
        }
    }
    return coprime;
}

fn beers(x: i32) {
    if x == 1 {
        println!("{} bottle of beer on the wall,", x);
        println!("{} bottle of beer.", x);
        println!("Take one down, pass it around,");
        println!("No bottles of beer on the wall,");
        println!("");
    } else {
        println!("{} bottles of beer on the wall,", x);
        println!("{} bottles of beer.", x);
        println!("Take one down, pass it around,");
        println!("{} bottles of beer on the wall,", x - 1);
        println!("");
    }
}

fn main() {
    for i in 0..=100 {
        if isprime(i) == true {
            println!("The number {} is prime!", i);
        } else {
            println!("The number {} is not prime!", i);
        }
    }

    for i in 0..=100 {
        for j in 0..=100 {
            if coprimes(i, j) == true {
                println!("The numbers {} and {} are  coprimes!", i, j);
            } else {
                println!("The numbers {} and {} are not coprimes!", i, j);
            }
        }
    }

    let mut index = 99;
    while index > 0 {
        beers(index);
        index = index - 1;
        if index == 0 {
            println!("No bottles of beer on the wall,");
            println!("No bottles of beer.");
            println!("Go to the store, buy some more,");
            println!("99 bottles of beer on the wall.");
        }
    }
}
