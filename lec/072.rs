

fn main() {
    let my_closure = || println!("calling closure");
    let print_x = |num: i32| {
        println!("x is {}", num);
    };

    my_closure();
    print_x(3 + 1);
}
