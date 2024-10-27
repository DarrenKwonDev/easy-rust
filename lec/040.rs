
#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog
}

// enum에 붙는 impl
impl Animal {
    // 연관함수
    fn create_cat(age: u8) -> Self {
        Self{age, animal_type: AnimalType::Cat}
    }

    fn print(&self) {
        println!("I am a {:?}, {} years old", self.animal_type, self.age);
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
    }
}

fn main() {
    let mut cat = Animal::create_cat(3);
    cat.print();
    cat.change_to_dog();
    cat.print();
}