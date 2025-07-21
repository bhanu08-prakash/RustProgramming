//1. Write a program to implement Type Casting Operator.

fn main() {

    let integer_value: i32 = 10;         
    let float_value: f64 = integer_value as f64;  

    let float_value2: f64 = 3.14;          
    let integer_value2: i32 = float_value2 as i32;  

    // Print the values
    println!("Original integer: {}", integer_value);
    println!("Converted to float: {}", float_value);
    
    println!("Original float: {}", float_value2);
    println!("Converted to integer: {}", integer_value2);
}
