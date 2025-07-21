//2. Write a program to determine if a number is even or odd

use std::io;

fn main() {
    println!("Please enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Please enter a valid number");

    if number % 2 == 0 {
        println!("The number {} is even.", number);
    } else {
        println!("The number {} is odd.", number);
    }
}
