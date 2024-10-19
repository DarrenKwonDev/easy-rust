fn main() {
    println!("size of char : {}", std::mem::size_of::<char>()); // rust에서 char는 4바이트

    /*
        len이 문자열의 길이가 아니라 "바이트 수"
    */
    println!("length of \"a\" : {}", "a".len());  // 바이트 수 반환. 1
    println!("size of \"a\" : {}", std::mem::size_of_val("a"));
    

    /*
        UTF-8 인코딩에서 문자의 바이트 수는 동적입니다
        ASCII는 1바이트쓰고 한국어는 3 ~ 4 바이트
    */
    println!("size of \"가\" : {}", "가".len()); // 바이트 수 반환. 3
    println!("size of \"가\" : {}", std::mem::size_of_val("가")); // 바이트 수 반환. 3
    println!("size of \"안녕!\" : {}", std::mem::size_of_val("안녕!")); // 바이트 수 반환. 7

    /*
        실제 문자의 갯수를 알기 위해서는 chars().count()를 이용해야.
    */
    println!("ascii chars: {}", "가".chars().count());  // 1
}
