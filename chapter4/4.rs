// 4. Write a function calculate_area_perimeter() that takes x and y( length and width of a
// rectangle) as a parameter to the function and returns a tuple (area, perimeter).

fn main(){
let x = 10;
let y = 10;
let res = calculate_area_perimeter(x,y);
println!("perimeter of length {} and breadth {} is {} ",x,y,res)
}
fn calculate_area_perimeter(l:i32, b:i32) -> i32{
let res = l * b;
res
}
