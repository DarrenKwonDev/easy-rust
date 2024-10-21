
fn main() {
    // array
    let seasons = ["spring", "summer", "aut", "win", "spring", "summer", "aut", "win"];

    // array를 자르면 slice가 됨. 
    // slice는 컴파일 시점에 크기를 알 수 없는 타입임.
    // println! 매크로는 내부적으로 Debug 트레이트를 구현한 Sized 타입을 기대합니다. 
    // 그런데 slice는 동적 크기 타입이므로 ... 없습니다.
    // 그래서 참조는 씁니다. 참조는 항상 고정된 크기(주소값의 크기)를 가지므로 Sized입니다.
    println!("{:?}", &seasons[0..2]); // 인덱스 0부터 1까지 (2 미포함)
    println!("{:?}", &seasons[..=2]); // 인덱스 0부터 인덱스 2까지 (2포함)
    println!("{:?}", &seasons[1..=3]); // 인덱스 1 부터 인덱스 3까지 (3포함)
}