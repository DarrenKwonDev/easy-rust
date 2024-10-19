
/*
    mutable 한 값은 mut 키워드가 필요함.
    shadowing은 안티 패턴이지만 컴파일러가 잡지는 않음
*/
fn main() {
    //------------------------------------
    // mut 변수
    let mut a = 3;
    println!("{}", a);
    
    a = 10;
    println!("{}", a);

    //------------------------------------
    // shadowing. 같은 변수를 다시 선언하는 경우.
    // 전의 변수에는 접근할 수 없게 됨.
    let temp = 10;
    println!("{}", temp);

    let temp = "str";
    println!("{}", temp);

    //------------------------------------
    // scope
    let _var = 0;
    {
        let _var = 3;
        // scope pop 되면서 _var은 stack 에서 제거됨
        // 변수가 스코프를 벗어나면 자동으로 메모리를 해제합니다
    }
    println!("{}", _var);
}