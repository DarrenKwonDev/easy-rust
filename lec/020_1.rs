
fn print_str(str: String) {
    println!("hello {}", str);
    // scope가 끝나면서 소유권을 가지고 있던 str은 drop됨
}

fn main() {
    let a = "asdf".to_string();
    print_str(a); // 이 시점에서 print_str 함수 내부로 소유권이 이전됨.
    
    // 이미 drop된 a를 사용하려고 하여 에러 발생함.
    print_str(a);
}