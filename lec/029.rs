use std::u8;

fn main() {
    let num:u8 = 5;
    if num == 5 {
        println!("simple if statement");
    } else if num == 6 {
        println!("simple else if statement");
    } else {
        println!("simple else statement");
    }

    // match 로 하려면 타입이 가능한 모든 경우의 수를 exhaustive하게 처리해야 함.
    let ret = match num {
        5 => 5 + 10,
        6..=10 => 20, // 6부터 10까지의 포함적(inclusive) 
        x if x > 10 => 30,  // 가드 조건 사용
        _ => u8::MAX // catch all
    };
    println!("{}", ret);
}