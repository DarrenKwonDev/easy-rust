
fn main() {
    let num = 15;
    let num_ref = &num;
    let _double_num_ref = &num_ref; // 레퍼런스의 레퍼런스 (cpp에서는 안되는데 rust엔 됨)
    let _go_crazy = &&&&&&&&&&&&&&&&&&&&_double_num_ref;
    
    // println! 매크로나 다른 포맷팅 상황에서 레퍼런스를 사용하면, Rust는 자동으로 그 값을 역참조합니다.
    println!("{}", *num_ref);
    println!("{}", *_double_num_ref); 
    println!("{}", _go_crazy);
}