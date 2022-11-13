fn main() {
    let thing_to_match_over = 4;
    match thing_to_match_over {
        num if num == 100 => println!("Im the large number"),
        0 => println!("Zero"),
        num @ 1..=5 if num == 4 => println!("4"),
        1..=5 => println!("Between 1 and 5"),
        _ => println!("Big Number"),
    }
}
