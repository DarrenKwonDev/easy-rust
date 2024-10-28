use std::fmt::Display;

fn compare_and_print<T: Display, U: Display + PartialOrd>(statement: T, num_1:U, num_2: U) {
    println!("{}! is {} greater than {}? {}", statement, num_1, num_2, num_1 > num_2);
}

fn compare_and_print2<T, U>(statement: T, num_1:U, num_2: U)
    where T: Display, U: Display + PartialOrd {
    println!("{}! is {} greater than {}? {}", statement, num_1, num_2, num_1 > num_2);
}

fn main() {
    compare_and_print("yo", 3, 4);
    compare_and_print2("yo", 3, 4);
}