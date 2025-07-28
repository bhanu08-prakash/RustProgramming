//3. Write a program to demonstrate the Pass by Value and Pass by Reference

fn main(){
let val = 4;
pbv(val);
pbrf(&val);
}

fn pbv(n:i32){
let m = n*n;
println!("pass by value is {} ",m)
}

fn pbrf(n : &i32){
let m = *n * *n;
println!("pass by reference value is {} ",m);
}

