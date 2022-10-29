const BOOK_PRICE: u32 = 800;

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut price = books.len() as u32 * BOOK_PRICE;
    for len in 2..6 {
        let groups = create_groups(books, len);
        let group_price = groups.iter().fold(0, |acc, itm| acc + get_group_price(itm));
        if group_price < price {
            price = group_price;
        }
    }

    price
}

// calculate the price of each group
// we assume each group contains different items
// group supposed to be between 1 - 5 in size
fn get_group_price(group: &[u32]) -> u32 {
    match group.len() {
        5 => 5 * (BOOK_PRICE - (BOOK_PRICE * 25 / 100)),
        4 => 4 * (BOOK_PRICE - (BOOK_PRICE * 20 / 100)),
        3 => 3 * (BOOK_PRICE - (BOOK_PRICE * 10 / 100)),
        2 => 2 * (BOOK_PRICE - (BOOK_PRICE * 5 / 100)),
        1 => BOOK_PRICE,
        _ => unreachable!("How did I get here?"),
    }
}

// create a grouping of books
// group_len indicates the max group size
// duplicate item puts in a different group
fn create_groups(books: &[u32], group_len: usize) -> Vec<Vec<u32>> {
    let mut groups: Vec<Vec<u32>> = vec![];
    assert!(group_len > 0 && group_len < 6);
    'outer: for book in books {
        for group in groups.iter_mut() {
            if group.len() < group_len && !group.contains(book) {
                group.push(*book);
                continue 'outer;
            }
        }
        groups.push(vec![*book]);
    }

    groups
}
