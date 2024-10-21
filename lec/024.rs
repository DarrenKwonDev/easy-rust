
fn main() {
    let arr = ["one", "two"];
    let arr2 = ["one", "two", "five"];
    let arr3 = ["one", "two"];

    // arr와 arr2는 다른 타입.
    // [&str; 2], [&str; 3] 로 각각 다른 타입
    // can't compare `[&str; 2]` with `[&str; 3]`
    // println!("{}", arr == arr2);

    // true.
    // c 배열처럼 포인터가 아니라 배열의 내용을 비교함.
    // cpp로 치면 std::array, std::vector 간 operator= 연산임.
    // 내부 원소가 전부 같다면 true임.
    println!("{}", arr == arr3);

    // buffer 선언할 때 유용함
    let buf = [0;1024];
    println!("{}, {:?}", buf.len(), buf); // 1024, [0, 0, ...]
}