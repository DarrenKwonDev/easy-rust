
fn main() {
    let c = "안녕하세요";
    println!("cnt : {}", c.chars().count());

    c.char_indices().for_each(|(index, charr)| {
        println!("{}{}", index, charr);
    });
}