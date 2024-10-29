
fn check_error(input: i32) -> Result<(), ()> {
    if input %2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    let variable = check_error(3);
    
    if variable.is_ok() {
        println!("ok");
    }
    if variable.is_err() {
        println!("err");
    }

    match variable {
        Ok(_) => println!("ok"),
        Err(_) => println!("err"),
    }

    variable.unwrap(); // err의 경우 panic!
}