
struct Animal {
    name: String
}

trait Canine {
    fn bark(&self);
    fn run(&self);
}

impl Canine for Animal {
    fn bark(&self) {
        println!("bark bark");
    }
    
    fn run(&self) {
        println!("run");
    }
}

fn main() {
    let dog = Animal{name: String::from("mydog")};
    dog.bark();
    dog.run();
}