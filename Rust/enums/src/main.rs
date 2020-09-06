fn main() {
    let one = One(20, 20);
    let two = Two{x:10, y:10};
    println!("One: {}, {}", one.0, one.1);
    println!("Two: {}, {}", two.x, two.y);
}

struct One(u16, u16);
struct Two { x: u16, y: u16 }

enum ForTest {
    One(One),
    Two(Two),
}
