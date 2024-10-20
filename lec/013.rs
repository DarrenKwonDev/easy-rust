
fn main() {
    print!("this");
    print!("this\n");
    print!("this\n");

    // raw text. r# ... #
    println!(r#"C:\Users\drkup\Documents\priceless"#);

    // second와 first 사이의 tab 또한 분리되어서 보임.
    // 붙여서 쓰고 싶다면 third 처럼 붙여야 함.
    println!("first
    second
third");

    let a = &10;
    println!("{}", a);   // 자동 역참조되어 10 출력
    println!("{:p}", a); // 실제로 메모리 주소를 출력하기 위해서는 :p 필요

    let b = 11234;
    println!("{:X}", b); // 16진수
    println!("{:b}", b); // 이진수

    /*
        ^ 중앙 < 왼쪽 > 오른쪽
    */
    let title = "NEWS";
    println!("{:-^30}", title); // 30글자, pad with -, center align

    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // {15글자 path with 공백, left align} {15글자 path with 공백, right align}

    let city1 = "seoul"; let city2 = "tokyo";
    println!("{0:-<15}{1:->15}", city1, city2); // {15글자 path with -, left align} {15글자 path with -, right align}

}