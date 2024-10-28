
// std::fmt::Display traits로 제네릭을 제한함.
// typescript의 extends 와 비슷한 개념임.
fn give_thing<T: std::fmt::Display>(input: T) -> T {
    input
}

fn main() {
    let x = give_thing("hello".to_string());
    println!("{}", x);
}