//1. Write a program to Find The Factorial using functions. 

fn main(){
let x = 5;
fact(5);
}

fn fact(n:i32){
let mut f = 1;
for i in 1…n+1{
f = f * i;
}
println!("factorial of {} is {} ",n,f);
}


