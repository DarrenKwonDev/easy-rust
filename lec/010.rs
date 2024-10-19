fn print_number(one: i32, two: i32) -> i32 {
    let multiplied_by_ten = {
        let first_number = 10;

        // ; 없이 마지막에 선언하였음
        // 마지막 표현식(세미콜론 없는)이 블록 전체의 값이 됩니다.
        first_number * one * two
    };

    multiplied_by_ten
}

fn main() {
    let res = print_number(2, 7);
    println!("{}", res);
}
