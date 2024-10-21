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
                let resource = i18n::fluent::parse(SOURCE);
                assert_eq!(target, resource);
            }
        )*
    };
}

tests![
    comments,
    eof_comment,
    eof_empty,
    eof_id_equals,
    eof_id,
    eof_junk,
    junk,
];
