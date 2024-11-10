
// https://doc.rust-lang.org/std/iter/trait.Iterator.html

fn main() {
    let vec = vec!['a', 'b', '가', '나'];
    let mut vec_iter = vec.iter();

    // println!("{:?}", vec_iter.next());
    // println!("{:?}", vec_iter.next());
    // println!("{:?}", vec_iter.next());
    // println!("{:?}", vec_iter.next());
    // println!("{:?}", vec_iter.next());

    assert_eq!(vec_iter.next(), Some(&'a'));
}