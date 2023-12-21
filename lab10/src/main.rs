use std::cell::RefCell;
use std::io;
use std::io::Write;
#[derive(Debug, Clone, Copy)]

struct Prime {
    number: u32,
    prime_flag: bool,
}
#[derive(Debug)]
struct Cache {
    computed_primes: RefCell<Vec<Prime>>,
}
const CACHE_SIZE: usize = 10; //can be changed to store as many number as we like

impl Cache {
    fn new() -> Cache {
        Cache {
            computed_primes: RefCell::new(Vec::with_capacity(CACHE_SIZE)),
        }
    }
    fn add(&self, prime: u32, flag: bool) {
        if self.computed_primes.borrow_mut().len() == CACHE_SIZE {
            self.computed_primes.borrow_mut().remove(0);
        }

        self.computed_primes.borrow_mut().push(Prime {
            number: prime,
            prime_flag: flag,
        });
    }

    fn in_cache(&self, prime: u32) -> Option<bool> {
        let mut value: Option<u32> = None;
        for i in self.computed_primes.borrow().iter() {
            if i.number == prime {
                value = Some(prime);
            }
        }
        if value.is_some(){
            let index = self
                .computed_primes
                .borrow()
                .iter()
                .position(|r| r.number == value.unwrap())
                .unwrap();
            let value = self.computed_primes.borrow()[index];
            self.computed_primes.borrow_mut().insert(CACHE_SIZE, value);
            self.computed_primes.borrow_mut().remove(index);
            return Some(self.computed_primes.borrow()[index].prime_flag);
        }

        None
    }

    fn print_cache(&self) {
        print!("\n CACHE:[");
        for prime in self.computed_primes.borrow().iter() {
            print!("{}-{} ", prime.number, prime.prime_flag);
        }
        print!("]\n");
    }
}
fn is_prime(x: u32) -> bool {
    if x == 1 {
        return false;
    }
    for i in 2..(x / 2 + 1) {
        if x % i == 0 {
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let cache = Cache::new();
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let number: u32 = buffer.trim().parse().unwrap();

        if cache.in_cache(number).is_some() {
            if cache.in_cache(number).unwrap(){
                println!("\n{} is prime!", number);
            } else {
                println!("\n{} is not prime!", number);
            }
        } else {
            if is_prime(number){
                println!("\n{} is prime!", number);
            } else {
                println!("\n{} is not prime!", number);
            }

            cache.add(number, is_prime(number));
        }
        cache.print_cache();
        io::stdout().flush().unwrap();
    }
}
