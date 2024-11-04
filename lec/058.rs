
#[derive(Debug)]
struct Book {
    title: String,
    year: u16
}

fn main() {
    let my_book = Book {
        title: "Some title".to_string(),
        year: 1919
    };
    let my_book2 = Book {
        title: "Book 2".to_string(),
        year: 2020
    };

    let width = 10;
    // println!("{my_book:?} {:#?}", my_book2);

    /*
        : 포매팅지정자
        포매팅지정자 다음 오는 문자 : 이걸로 채우겠다.
        ^ 가운데 정렬 < 왼쪽 정렬 > 오른쪽 정렬
        width$ width 만큼 너비 지정
        ? debug format사용
    */
    println!("{my_book:ㅋ^width$?}");
}