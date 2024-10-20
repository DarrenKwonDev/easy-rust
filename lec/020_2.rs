
fn print_str(str: &String) {
    println!("hello {}", str);
    // scope가 끝나면서 소유권을 가지고 있던 str은 drop됨
}

fn main() {
    let a = "asdf".to_string();

    // ref로 넘기면 소유권을 넘기지 않음.
    print_str(&a);
    print_str(&a);
}