fn main() {
    let my_vec = vec![1, 2, 3];

    let double_vec = my_vec
        .iter()
        .map(|num| num * 2)
        .map(|num| num * 2)
        .map(|num| num * 2)
        .map(|num| num * 2)
        .collect::<Vec<i32>>();

    println!("{:?}", double_vec);

    double_vec
        .iter()
        .enumerate()
        .for_each(|(index, number)| {
            println!("{index} {number}");
        });
}


