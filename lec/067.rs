use std::fmt::Display;


// T 타입의 값을 &str (문자열 슬라이스)로 변환할 수 있어야 함
fn print_it<T : Display + AsRef<str>>(input: T) {
    println!("{input}");
}

fn main() {
    print_it("hello");
    // print_it(3); // compile err
}