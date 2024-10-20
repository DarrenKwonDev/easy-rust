fn main() {
    let a = "asdf";
    let b = &mut a;


    let a = 3; // a shadowing
    b = "abb"; // ?? 안 먹네?
    println!("{}, {}", b, a); // b가 아직 살아 있다!
}