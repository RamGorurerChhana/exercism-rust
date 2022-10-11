#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // if both lists are empty or equal
    if _first_list.iter().eq(_second_list.iter()) {
        return Comparison::Equal;
    }

    // if first list is empty then it is sublist
    if _second_list.len() > 0 && _first_list.len() == 0 {
        return Comparison::Sublist;
    }

    // if first list non empty and second list is empty then it is superlist
    if _first_list.len() > 0 && _second_list.len() == 0 {
        return Comparison::Superlist;
    }

    // if second list is greater in size of first list then it might be sublist
    // attemp to match against window slice of second list with first list
    // if matched against any window slice then it is a sublist
    if _second_list.len() > _first_list.len() {
        for list in _second_list.windows(_first_list.len()) {
            if list.iter().eq(_first_list.iter()) {
                return Comparison::Sublist;
            }
        }
    }

    // if first list is greater in size of second list then it might be superlist
    // attempt to match against window slice of first list with second list
    // if any window slice matches then it is a superlist
    if _first_list.len() > _second_list.len() && _second_list.len() > 0 {
        for list in _first_list.windows(_second_list.len()) {
            if list.iter().eq(_second_list.iter()) {
                return Comparison::Superlist;
            }
        }
    }

    // if none of the above are true then lists are unequal
    Comparison::Unequal
}
