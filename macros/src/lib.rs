#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    }; 
    ($($x:expr => $y:expr),+ $(,)?) => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($x, $y);)+ 
        map
    }};
}





