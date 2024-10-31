
fn main() {
    let my_vec = vec![1, 2, 3];
    let get_one = my_vec.get(0);
    let get_ten = my_vec.get(11);

    println!("{:?} {:?}", get_one, get_ten);

    for index in 0..10 {

        // if let은 match 표현식의 간단한 형태로, 하나의 패턴만 검사하고 싶을 때 유용합니다.
        if let Some(number) = my_vec.get(index) {
            println!("The number is {}", number);
        }

        // if let 구문이 아닌 match로 하려면 아래와 같이 다 확인해야 함.
        // match my_vec.get(index) {
        //     Some(number) => println!("The number is {}", number),
        //     None => ()
        // }
    }

}