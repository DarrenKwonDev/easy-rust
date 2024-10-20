/*
    & : immutable ref, shared ref, immutable borrow
    &mut : mutable ref, unique ref, mutable borrow
*/

fn main() {
    let mut num = 10;
    let num_ref = &num; // immutable ref (immutable borrow)

    // cannot borrow `num` as mutable because it is also borrowed as immutable
    // 즉, mutable borrow 할 수 없다 왜냐하면 immutable borrow를 이미 했기 때문이다.
    let num_change = &mut num; // mutable ref를 만드려고 시도. 실패함.
    *num_change += 10;
    
    println!("{}", num_ref);
}