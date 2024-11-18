use std::collections::HashMap;

fn main() {
    let a = vec![0, 1, 2, 3, 4, 5, 6];
    let b = vec!["zero", "one", "two", "three", "four"];

    let new = a
        .into_iter()
        .zip(b.into_iter())
        .collect::<HashMap<i32, &str>>();

    // 짧은 것을 기준으로 zip됨.
    new.iter().for_each(|(x, y)| {
        println!("{x} {y}")
    });
}