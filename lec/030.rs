
fn main() {
    let sky = "cloudy";
    let temp = "warm";

    // 각 match는 하나만 만족해도 break 됨. 
    // 여러개가 만족한다면 순서에 의존하므로 주의할 것.
    match (sky, temp) {
        _ => println!("not sure"), // catch all이 먼저 나온다면, 이것에 먼저 걸리므로 아래가 실행 안됨
        ("cloudy", "cold") => println!("cloudy cold"),
        ("cloudy", "warm") => println!("cloudy warm")
    }
}