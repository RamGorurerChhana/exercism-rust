
pub fn map<F, T, U>(input: Vec<T>, mut function: F) -> Vec<U> 
    where F: FnMut(T) -> U
{
    let mut v = vec![];
    for e in input.into_iter() {
        v.push(function(e));
    }

    v
}
