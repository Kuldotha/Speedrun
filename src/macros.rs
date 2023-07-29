pub mod macros {
    #[macro_export]
    macro_rules! dict {
        ($(($key:expr, $value:expr)),*) => {
            {
                #[allow(unused_mut)]
                let mut map = Dictionary::new();
                $(map.insert($key, $value);)*
                map
            }
        };
    }
}
