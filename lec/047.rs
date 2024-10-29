

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let index = take_fifth(new_vec);
    println!("{:?}", index);

    // match로 해결하는 법
    // match index {
    //     Some(number) => println!("{}", number),
    //     None => println!("None"),
    // }

    // unwrap()을 사용하는 법
    // println!("{}", index.unwrap()); // None 이면 panic을 일으킴

    // is_some으로 체크하는 방법
    // if index.is_some() {
    //     println!("{}", index.unwrap());
    // }

    // expect로 panic에 원하는 메시지도 출력하기
    // index.expect("you need vec has more than 5 len");
}