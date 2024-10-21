
fn main() {
    let a = "rabbit".to_string();
    let b = "cat".to_string();

    // 일반적인 vec 생성 방법
    let mut my_vec: Vec<String> = Vec::new();
    my_vec.push(a);
    my_vec.push(b);

    // 조금 더 간편하게 vec! 매크로를 사용할수도
    let another_vec = vec!["dog", "bird"];

    println!("{:?}, {}, {}", my_vec, my_vec.len(), my_vec.capacity());
    println!("{:?}, {}, {}", another_vec, another_vec.len(), another_vec.capacity());
}