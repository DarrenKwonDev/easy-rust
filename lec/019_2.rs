fn main() {
    let a = "asdf";
    let b = &a;

    let a = 3; // a shadowing
    println!("{}, {}", b, a); // asdf, 3 즉, b가 아직 살아 있다! 
}