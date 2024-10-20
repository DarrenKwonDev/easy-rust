/*
    & : immutable ref, shared ref
    &mut : mutable ref, unique ref
*/

fn main() {
    let mut my_number = 9;

    /*
    가변 참조가 존재하는 동안에는 원본 변수를 직접 수정할 수 없음.
    my_number의 mutable ref에 해당하는 num_ref 로만 값을 변경할 수 있습니다.
    */
    {
        let num_ref = &mut my_number;

        // dereference 하여 원본 변경 가능
        *num_ref = 3;
        println!("{}", num_ref);
    }
    
    // scope를 벗어나 mut ref는 drop 되었고, 이제 원본을 통해 직접 변경 가능    
    my_number = 11;
    println!("{}", my_number);

}