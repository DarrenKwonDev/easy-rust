
// https://doc.rust-lang.org/std/iter/trait.Iterator.html

#[derive(Debug)]
enum LibraryType {
    City,
    Country
}
#[derive(Debug)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }

    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            books: Vec::new()
        }
    }
}

impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.books.pop() {
            Some(book) => Some(book + "book find"),
            None => None,
        }
    }
}


fn main() {
    let mut my_lib = Library::new();
    my_lib.add_book("asdf");
    my_lib.add_book("aaaa");
    my_lib.add_book("ghhh");
    my_lib.add_book("momo");

    let filtered_books: Vec<String> = my_lib
        .map(|book| book.to_uppercase())
        .filter(|book| book.contains("BOOK"))
        .collect();
    println!("2. Filtered and mapped books: {:?}", filtered_books);
}