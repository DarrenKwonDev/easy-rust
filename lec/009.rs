/*
    ()  = empty tuple, unit type(void)
    fn something() -> () { ... }  꼴이라면 void 반환 함수라고 보면 됨.
*/
fn _ret_eight() -> i32 {
    8 // 8이 반환됨
}

fn empty_tuple() {
    8; // 세미콜론을 넣어서 그냥 8이 평가되고 끝난 것임.
    
    // 최종적으로 return 되는 것이 없으므로 ()가 반환이 됨. 
}
fn main() {
    let tup = empty_tuple();
    println!("debug print : {:?}", tup); // () 는 debug print로만 출력됨.
}
