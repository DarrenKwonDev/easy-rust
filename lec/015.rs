
/*
std::string::String
// +---------------+
// | ptr           | -> (힙의 문자 데이터를 가리킴)
// +---------------+
// | len (5)       |
// +---------------+
// | capacity (5)  |
*/

fn main() {
    let mut name: String = "david cho".to_string();
    name.push('!');
    name.push_str(" yep");
    name.pop();
    println!("name is {}, cap is {}, len is {}", name, name.capacity(), name.len());


    // re-allocation을 피하기 위해서 cap을 지정한채로 String을 생성하는 방법.
    let mut name2 = String::with_capacity(20);
    name2.push_str("david cho! yep");
    println!("name is {}, cap is {}, len is {}", name2, name2.capacity(), name2.len());
}
