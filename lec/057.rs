use std::num::ParseIntError;

fn parse_int(input: &str) -> Result<i32, ParseIntError> {
    // ? 가 아니라면 Result<i32, ParseIntError>
    // ? 가 붙음으로써 i32 타입으로 추론됨

    /*
    ?가 없다면 아래처럼 match 작성을 해야 함.
    match input.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(e) => Err(e)
    }
    */
    let parsed_num = input.parse::<i32>()?;
    Ok(parsed_num)
}

fn main() {
    for item in vec!["3", "3.0", "asdf"] {
        let parsed = parse_int(&item);
        println!("{:?}", parsed);
    }
}