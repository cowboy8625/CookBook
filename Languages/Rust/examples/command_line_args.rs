use std::env;
//I was not expecting to have to borrow size
#[warn(non_snake_case)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = 1;
    if &args.len() > &size {
        println!("{}", args[1]);
    }
}
