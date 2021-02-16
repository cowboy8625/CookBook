#[derive(Debug)]
struct Foo1;
#[derive(Debug)]
struct Foo2;
#[derive(Debug)]
struct Foo3;

#[derive(Debug)]
enum FooBros {
    One(Foo1),
    Two(Foo2),
    Three(Foo3),
}

fn main() {
    use FooBros::*;
    let vec = vec![One(Foo1), Two( Foo2 ), Three( Foo3 )];
    dbg!(vec);
}


fn _option_example(num: i32) -> Option<i32> {
    if num > 0 {
        return Some(num);
    }
    None
}

fn _result_example(num: i32) -> Result<i32, String> {
    if num > 0 {
        return Ok(num);
    }
    Err("This is a negative number".to_string())
}
