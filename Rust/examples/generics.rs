use std::ops::{Add, AddAssign, Sub, SubAssign};

fn main() {
    let num = add(4, 6);
    println!("num = {}", num);
    let num1 = sub(4, 6);
    println!("num = {}", num);

    let mut p = Point { x: 30, y: 50 };
    println!("Point({}, {})", p.x, p.y);

    p.add(5, 5);
    println!("Point({}, {})", p.x, p.y);

    p.sub(1000, 1000);
    println!("Point({}, {})", p.x, p.y);
}

fn sub<T>(x: T, y: T) -> T
where
    T: Sub<Output = T>,
{
    x - y
}

fn add<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn add(&mut self, x: T, y: T)
    where
        T: AddAssign,
    {
        self.x += x;
        self.y += y;
    }
    fn sub(&mut self, x: T, y: T)
    where
        T: SubAssign,
    {
        self.x -= x;
        self.y -= y;
    }
}
