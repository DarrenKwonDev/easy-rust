
/*
    [String]
    힙에 할당된 문자열로, 소유권을 가집니다. 변수가 스코프를 벗어나면 자동으로 메모리가 해제됩니다.
    소유권 규칙에 따라 관리됩니다.

    [&str]
    문자열 슬라이스로, 다른 문자열 데이터(String이나 정적 문자열)를 참조합니다. 소유권이 없습니다.
    명시적인 라이프타임 지정이 필요할수도 있음

    [변환]
    String에서 &str로: &를 사용하여 쉽게 변환 가능 (&String은 &str로 자동 변환)
    &str에서 String으로: to_string() 또는 String::from()을 사용
*/
fn main() {
    let mut name = "David".to_string();
    name.push_str(" cho");

    let mut other_name = String::from("mama");
    other_name.push('!');
    
    println!("{}", name);
    println!("{}", other_name);
}
