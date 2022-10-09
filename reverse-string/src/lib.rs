pub fn reverse(input: &str) -> String {
    // Note: collect method does not need type annotation here
    // baecause it is the return statement in the function
    // so rust is able to infer the type from the return type of the function.
    input.chars().rev().collect()
}
