
fn print_number(number: i32) {
    println!("{}", number);
}

fn print_str(str: String) {
    println!("{}", str);
}

fn main() {
    let num = 8;
    print_number(num); // copy traits 가 있으므로 함수 내부에 복사되어 전달됨.

    let str = "asdf".to_string();
    print_str(str); // copy traits가 없음. 소유권이 넘어감.\
    //println!("{}", str); // error.소유권이 이미 없음. (borrow of moved value)

    let str2 = "xxxx".to_string();
    print_str(str2.clone());  // clone 하여 넘김. 원본인 str2의 소유권은 넘어가지 않음.
    println!("{}", str2); // 소유권이 넘어가지 않았으므로 아직 유효한 값
}