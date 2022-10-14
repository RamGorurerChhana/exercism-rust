pub fn find<T, U>(array: T, key: U) -> Option<usize>
where
    T: AsRef<[U]>,
    U: Ord,
{
    let len = array.as_ref().len();
    // check for first element in the array
    if key == *(array.as_ref().first()?) {
        return Some(0);
    }

    // check for last element in the array
    if key == *(array.as_ref().last()?) {
        return Some(len - 1);
    }

    // if array have less than 2 elements then return None
    if len < 2 {
        return None;
    }

    let mid = len / 2;
    // check mid element
    let mid_element = array.as_ref().get(mid)?;
    if key == *(mid_element) {
        Some(mid)
    } else if key < *(mid_element) {
        let idx = find(&array.as_ref().get(1..mid)?, key)?;
        Some(1 + idx)
    } else {
        let idx = find(&array.as_ref().get(mid + 1..len - 1)?, key)?;
        Some(mid + 1 + idx)
    }
}
