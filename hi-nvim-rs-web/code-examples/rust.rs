//! Calculate A+B.
//!
//! From: https://rosettacode.org/wiki/A+B
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("stdin");

    let mut i: i64 = 0;
    for word in line.split_whitespace() {
        i += word
            .parse::<i64>()
            .expect("interpret input as numbers");
    }
    println!("{}", i);
}
