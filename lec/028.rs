
fn main() {
    // tuple 선언
    let my_tup = (8, "asdf", 1.5, vec![1, 2, 3]);
    println!("{} {} {} {:?}", my_tup.0, my_tup.1, my_tup.2, my_tup.3);

    // tuple도 하나의 타입니까 묶어서 vec 원소로 만드는 등 활용.
    let _vec_of_tup = vec![([1, 2], 3), ([1, 2], 3)];
    
    // deconstruct
    let (_a, _b, _c, _d) = my_tup; 
}