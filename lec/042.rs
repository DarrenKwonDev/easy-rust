

struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool
}

struct Person2 {
    name: String,
    height: u8
}

impl Person2 {
    fn from_person(input: Person) -> Self {

        // 나머지는 .. 로 deconstruct 시 무시가능.
        let Person { name, height, .. } = input;
        Self{name, height}
    }
}

fn main() {
    let mut papa = Person {
        name: "papa_doc".to_string(),
        real_name: "clarence".to_string(),
        height: 170,
        happiness: false
    };

    // deconstruct
    // javascript의 그것과 같음.
    let Person {name: papa_name, real_name: r, height: h, happiness: hp} = papa;
    println!("{}, {}, {}, {}", papa_name, r, h, hp);
}