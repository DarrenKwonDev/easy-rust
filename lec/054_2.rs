use std::collections::HashSet;

fn main() {
    let many_numbers = vec![1, 1, 1, 1, 2];

    let mut number_hs = HashSet::new();

    for num in many_numbers {
        number_hs.insert(num);
    }

    println!("unique num cnt : {}", number_hs.len());
}