
// https://doc.rust-lang.org/std/iter/trait.Iterator.html

fn main() {
    let vec1 = vec![1, 2, 3];

    // iter()      -> &T     (불변 참조)
    let vec1_a = vec1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    // vec1은 여전히 사용 가능

    // into_iter() -> T      (소유권)
    let vec1_b: Vec<i32> = vec1.into_iter().map(|x| x * 10).collect();
    // vec1은 이제 사용 불가능 (소유권이 이동됨)


    // iter_mut()  -> &mut T (가변 참조)
    let mut vec2 = vec![10, 20, 30];
    vec2.iter_mut().for_each(|x| *x += 100);
    // 가변 참조(&mut T)를 생성 원본 컬렉션의 값을 직접 수정 가능


    println!("{:?}", vec1_a); // iter()로 만든 새로운 벡터
    println!("{:?}", vec1_b); // into_iter()로 만든 새로운 벡터
    println!("{:?}", vec2); // iter_mut()로 직접 수정된 원본 벡터
}