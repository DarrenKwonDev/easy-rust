fn main() {
    let num = 3;

    // pattern binding operator
    // @pattern
    match num {
        res @ 0..=10 => println!("match with {}", res),
        _ => println!("not between 0 ~ 10")
    }
}