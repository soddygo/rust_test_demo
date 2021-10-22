pub(crate)
fn apply(value: i32, f: fn(i32) -> i32) -> i32 { f(value)}
pub(crate)
fn square(value: i32) -> i32 { value * value}
pub(crate)
fn cube(value: i32) -> i32 { value * value * value}


fn main(){
    println!("apply square: {}", apply(2, square)); println!("apply cube: {}", apply(2, cube));

}