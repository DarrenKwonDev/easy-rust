fn main() {
    let a = "asdf";
    let b = &a;

    let a = 3; // a shadowing
    println!("{}, {}", b, a); // b가 아직 살아 있다!
}