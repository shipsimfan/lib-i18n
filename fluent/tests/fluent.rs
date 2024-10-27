//! Fluent tests

macro_rules! tests {
    [$($name: ident,)*] => {
        mod fluent {
            $(pub mod $name;)*
        }

        $(
            #[test]
            fn $name() {
                const SOURCE: &str = include_str!(concat!("fluent/", stringify!($name), ".ftl"));

                let target = fluent::$name::create_target();
                let resource = ::fluent::parse(SOURCE);
                assert_eq!(target, resource);
            }
        )*
    };
}

tests![
    any_char,
    comments,
    cr_multikey,
    cr_multilinevalue,
    eof_comment,
    eof_empty,
    eof_id_equals,
    eof_id,
    eof_junk,
    eof_value,
    junk,
    mixed_entries,
    sparse_entries,
    special_chars,
    variables,
];
