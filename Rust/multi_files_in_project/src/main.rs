use std::ops::{Add, Sub};
mod point;

fn main() {
    let num1 = add(4, 6);
    println!("num 1 = {}", num1);
    let num2 = sub(4, 6);
    println!("num 2 = {}", num2);
    let mut p = point::Point { x: 64, y: 32 };
    println!("Point({}, {})", p.x, p.y);
}


fn sub<T>(x:T, y:T) -> T where T: Sub<Output=T> {
    x - y
}
fn add<T: Add<Output = T>>(x:T, y:T) -> T {
    x + y
}

