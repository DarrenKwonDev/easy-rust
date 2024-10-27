
fn main() {
    let mut cnt = 0;

    // loop에 이름 지정
    let ret = 'first_loop: loop {
        cnt += 1;
        
        loop {
            if cnt >= 50 {
                // 지정된 이름의 loop를 벗어나되 cnt를 반환한다.
                break 'first_loop cnt;
            }
        }
    };

    println!("{}", ret);
}