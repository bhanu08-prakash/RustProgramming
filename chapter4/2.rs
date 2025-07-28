// 2. Write a function test_divisibility_by_3_4 which will check whether a given integer
// number is divisible by 3 or 4.
// a. If the number is divisible by both return 0
// b. If the number is divisible by 3 only return 1
// c. If the number is divisible by 4 only return 2
// d. If the number is not divisible by both, return -1


fn main(){
let x = 4;
let val = divi(x);
println!("value is {} ",val);
}

fn divi(n:i32)->i32{
if (n % 3 == 0) && (n % 4 == 0) {
0
}
else if n % 3 == 0 {
1
}
else if n % 4 == 0 {
2
}
else{
-1
}
}

