fn main() {
    let _a = 'A' as u8;
    let _c = 65 as char;
    /*
        C 에서 char는 1바이트(8비트) -> ASCII만 지원함
        rust에서 char는 32비트 (4바이트) -> 유니코드 지원을 위함.
        
        rust에서 ASCII만 지원하고 싶다면 u8을 써야.
    */
    
    println!("num is {} char is {}", _a, _c);

    // char가 실제로 4바이트다!
    println!("size of char : {}", std::mem::size_of::<char>());
}
