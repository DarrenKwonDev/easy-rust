
fn main() {
    // 숫자와 타입을 병기할 수 있다.
    let _a = 3u8; // 3 인데 u8 타입
    let _b = 3_u8; // 3 인데 u8 타입
    let _c = 3__________u8; // 3 인데 u8 타입

    // 숫자 사이에 _를 넣어 구분자로 사용할 수 있다.
    let _d = 1_000_000_000i64;

    // 정수 선언 없이 i32, float 선언 없이 f64로 추론됨.
    let num = 9;
    let num2  = 9.8;

    println!("res is {}", num + num2 as i32);
}
