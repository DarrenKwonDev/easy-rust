use std::cell::RefCell;


fn main() {
    let data = RefCell::new(String::from("hello"));

    // Multiple readers OK
    let r1 = data.borrow();
    let r2 = data.borrow();  // 여러 개의 불변 참조 가능
    println!("{} {}", r1, r2);

    // Single writer
    let mut w1 = data.borrow_mut();  // OK
    let mut w2 = data.borrow_mut();  // 패닉! 이미 가변 참조 있음

    // Reader + Writer 
    let r1 = data.borrow();          // 불변 참조
    let mut w1 = data.borrow_mut();  // 패닉! 불변 참조가 있는데 가변 참조 시도
}