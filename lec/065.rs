use std::fmt::Debug;

trait Prints {
    fn prints_something(&self) {
        println!("I like to print things");
    }
}

#[derive(Debug)]
struct Person;


// blanket trait impl
impl<T: Debug> Prints for T {}

fn main() {
    let p = Person;
    p.prints_something();

    let x = String::from("anything");
    x.prints_something();
}