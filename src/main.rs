use std::collections::HashMap;

fn main() {
    let vec = vec![
        "btc",
        "btc",
        "eth",
        "pepe"
    ];

    let mut hm = HashMap::new();

    for elem in vec {
        let cnt = hm.entry(elem).or_insert(0);
        *cnt += 1;
    }

    for (coin, cnt) in hm {
        println!("{} {}", coin, cnt);
    }
}