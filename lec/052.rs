use std::collections::HashMap;

fn main() {
    let alph = vec!["a", "b", "c"];
    let ko = vec!["ㄱ", "ㄴ", "ㄷ"];

    let mut hm = HashMap::new();

    for al in alph {
        hm.insert(al, "alphabet");
    }
    for k in ko  {
        hm.insert(k, "korean");
    }

    // println!("{}", hm["g"]); // panic

    println!("{:?}", hm.get("g")); // not panic. Option returned
}