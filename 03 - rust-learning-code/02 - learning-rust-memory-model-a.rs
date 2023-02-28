struct Book {
    pages: i32,
    rating: i32,
}

fn page_count(book: &Book) {
    println!("pages= {:?}, book.pages");
}

fn book_rating(book: &Book) {
    println!("rating= {:?}, book.rating");
}

fn main() {
    let book = Book {
        pages = 5,
        rating = 9,
    };
    page_count(&book);
    book_rating(&book);
}
