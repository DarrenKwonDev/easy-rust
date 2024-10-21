
fn loop_then_ret(mut counter: i32) -> i32 {
    loop {
        counter +=1;
        if counter % 50 == 0 {
            break;
        }
    }
    counter
}

fn main() {
    let num = {
        let x = 10;
        x + 3
    };
    
    println!("{}", loop_then_ret(num));
}