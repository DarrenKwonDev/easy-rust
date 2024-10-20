
// mutable ref 로, 소유권 이전 없이 가져오기만 함. 변경은 가능
fn add(country: &mut String) {
    country.push_str(" is great?");
    println!("{}", country);
}

// 소유권을 이전함(String 형을 그대로 가져옴)
// 그리고 함수 내부에서 country 함수는 이제 변경이 가능한 변수가 됨.
fn add2(mut country: String) {
    country.push_str(" is down sizing?");
    println!("{}", country);
}

fn main() {
    let mut country = "캐나다".to_string();
    add(&mut country);
    add2(country);
}