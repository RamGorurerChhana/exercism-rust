const BOOK_PRICE: u32 = 800;


pub fn lowest_price(books: &[u32]) -> u32 {
    unimplemented!(
        "Find the lowest price of the bookbasket with books {:?}",
        books
    )
}

fn get_zero_book_price() -> u32 {
    0
}

fn get_one_book_price() -> u32 {
    BOOK_PRICE
}

fn get_two_books_price(books: &[u32]) -> u32 {
    if books[0] == books[1] {
        2 * BOOK_PRICE
    } else {
        2 * (BOOK_PRICE - ((BOOK_PRICE * 5) / 100))
    }
}

fn get_three_books_price(books: &[u32]) -> u32{
    if books[0] != books[1] && books[0] != books[2] && books[1] != books[2] {
        3 * (BOOK_PRICE - ((BOOK_PRICE * 10) / 100))
    } else if books[0] == books[1] && books[1] == books[2] {
        3 * BOOK_PRICE
    } else {
        BOOK_PRICE + (2 * (BOOK_PRICE - ((BOOK_PRICE * 5) / 100)))
    }
}

// fn get_four_books_price(books: &[u32]) -> u32 {

// }


fn create_groups(books: &[u32], group_len: usize) -> Vec<Vec<u32>> {
    let mut groups = vec![];
    if group_len == 0 {
        return groups;
    }
    // let mut group = vec![];
    // for book in books {

    // }




    groups
}



