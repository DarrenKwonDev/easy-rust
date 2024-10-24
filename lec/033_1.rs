use std::mem;

#[derive(Debug)]
struct Country {
    pop: u32,
    capital: String,
    leader_name: String
}

fn main() {
    let pop = 35_000_000;
    let capital = "Seoul".to_string();
    let leader_name = "park".to_string();

    
    // 구조체 필드와 이름이 같아서 아래 처럼 쓸 수 있다.
    let country = Country {
        pop: pop,   // 굳이 따로 쓰자면 이렇게.
        capital,
        leader_name
    };

    // 4 + 24 + 24 = 52 바이트 (4바이트 + String 24 바이트 + String 24 바이트)
    println!("{:?}, {}", country, size_of_val(&country)); 

    // 분해 할당
    let another_country = Country {
        pop: 35_000,
        ..country
    };

    println!("{:?}, {}", another_country, size_of_val(&another_country));
}