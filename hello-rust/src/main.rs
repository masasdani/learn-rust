mod utils;
mod ids;

use std::{env, str::FromStr};

use crate::utils::generate_secure_random_string_lowercase;

fn main() {

    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    generate_secure_random_string_lowercase(10);

    println!("The greatest common divisor of {:?} is {}", numbers, d);
    
}


fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}


#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
}

#[test]
fn test_random_string() {
    let random_string = generate_secure_random_string_lowercase(10);
    assert_eq!(random_string.len(), 10);
}