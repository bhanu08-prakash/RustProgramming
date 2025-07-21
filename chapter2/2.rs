//2. Write a program to implement Borrowing and Dereferencing Operators

fn main() {
    let x = 10;
    let y = &x;

    println!("The value of x: {}", x);
    println!("The value of y (borrowed reference): {}", y);
    println!("Dereferencing y: {}", *y);
    println!("Value of x again: {}", x);
}
