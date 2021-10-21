mod demo1;
mod demo0;

use std::fs;

fn main() {

    demo0::convert_md();


    println!("apply square: {}", demo1::apply(2, demo1::square)); println!("apply cube: {}", demo1::apply(2, demo1::cube));
}
