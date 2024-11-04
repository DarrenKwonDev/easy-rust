use core::fmt;

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8
}

// https://doc.rust-lang.org/std/fmt/trait.Display.html
impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "custom display fmt func ({}, {})", self.name, self.age)
    }
}
fn main() {
    let mr_mantle  = Cat {
        name: "riggie mantle".to_string(),
        age: 4
    };
    
    println!("{mr_mantle}");
}