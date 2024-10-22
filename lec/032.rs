
// unit struct. byte 수가 0
struct FileDir;

// tuple struct
struct Color(u8, u8, u8);

// named struct
struct Country {
    pop: u32,
    capital: String,
    leader_name: String
}

fn read_dir(dir: FileDir) {
    // 스택에 있는 구조체의 크기를 반환
    println!("size of dir {}", std::mem::size_of_val(&dir)); // 0 를 출력함
}
fn main() {
    let x = FileDir;
    read_dir(x);

    let y = Color(20, 30, 50);
    println!("{}", y.1);

    let z = Country{
        pop: 35_000_000,
        capital: "test".to_string(),
        leader_name: "asdf".to_string()
    };
    println!("{} {} {}", z.pop, z.capital, z.leader_name);
}