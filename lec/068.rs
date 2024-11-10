fn main() {
    let new_vec = (1..=10).step_by(2).collect::<Vec<i32>>();

    println!("{:?}", new_vec);
}
