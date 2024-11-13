/// Creates a [`MessageKey`](crate::translation::MessageKey)
#[macro_export]
macro_rules! message_key {
    (
        $(#[$meta: meta])*
        $vis: vis $name: ident { $($arguments: tt)* } [
            $first_tag: ident => { $($first_message: tt)+ },
            $($tag: ident => { $($message: tt)+ },)*
        ]
    ) => {
        $(#[$meta])*
        $vis struct $name {
            $($arguments)*
        }

        impl $name {
            $crate::message!(#[allow(non_snake_case)] $name, $first_tag $($first_message)+);
            $($crate::message!(#[allow(non_snake_case)] $name, $tag $($message)+);)*
        }

        impl $crate::MessageKey for $name {
            type Arguments<'a> = $name;

            fn default_message<'a>() -> $crate::Message<Self::Arguments<'a>> {
                Self::$first_tag
            }

            fn try_get_message<'a>(
                language: &$crate::locale::LanguageTag,
            ) -> Option<$crate::Message<Self::Arguments<'a>>> {
                if language == $first_tag {
                    return Some(Self::$first_tag);
                }

                $(if language == $tag {
                    return Some(Self::$tag)
                })*

                None
            }
        }
    };

    (
        $(#[$meta: meta])*
        $vis: vis $name: ident<'a> { $($arguments: tt)* } [
            $first_tag: ident => { $($first_message: tt)+ },
            $($tag: ident => { $($message: tt)+ },)*
        ]
    ) => {
        $(#[$meta])*
        $vis struct $name<'a> {
            $($arguments)*
        }

        impl<'a> $name<'a> {
            $crate::message!(#[allow(non_snake_case)] $name, $first_tag $($first_message)+);
            $($crate::message!(#[allow(non_snake_case)] $name, $tag $($message)+);)*
        }

        impl<'a> $crate::MessageKey for $name<'a> {
            type Arguments<'b> = $name<'b>;

            fn default_message<'b>() -> $crate::Message<Self::Arguments<'b>> {
                Self::$first_tag
            }

            fn try_get_message<'b>(
                language: &$crate::locale::LanguageTag,
            ) -> Option<$crate::Message<Self::Arguments<'b>>> {
                if language == $first_tag {
                    return Some(Self::$first_tag);
                }

                $(if language == $tag {
                    return Some(Self::$tag)
                })*

                None
            }
        }
    };

    (
        $(#[$meta: meta])*
        $vis: vis $name: ident with $arguments: ident$(<$lifetime: lifetime>)* [
            $first_tag: ident => { $($first_message: tt)+ },
            $($tag: ident => { $($message: tt)+ },)*
        ]
    ) => {
        $(#[$meta])*
        $vis struct $name;

        impl $name {
            $crate::message!(#[allow(non_snake_case)] $arguments, $first_tag $($first_message)+);
            $($crate::message!(#[allow(non_snake_case)] $arguments, $tag $($message)+);)*
        }

        impl $crate::MessageKey for $name {
            type Arguments<'a> = $arguments$(<$lifetime>)*;

            fn default_message<'a>() -> $crate::Message<Self::Arguments<'a>> {
                Self::$first_tag
            }

            fn try_get_message<'a>(
                language: &$crate::locale::LanguageTag,
            ) -> Option<$crate::Message<Self::Arguments<'a>>> {
                if language == $first_tag {
                    return Some(Self::$first_tag);
                }

                $(if language == $tag {
                    return Some(Self::$tag)
                })*

                None
            }
        }
    };

    (
        $(#[$meta: meta])*
        $vis: vis $name: ident [
            $first_tag: ident => { $($first_message: tt)+ },
            $($tag: ident => { $($message: tt)+ },)*
        ]
    ) => {
        $(#[$meta])*
        $vis struct $name;

        impl $name {
            $crate::message!(#[allow(non_snake_case)] (), $first_tag $($first_message)+);
            $($crate::message!(#[allow(non_snake_case)] (), $tag $($message)+);)*
        }

        impl $crate::MessageKey for $name {
            type Arguments<'a> = ();

            fn default_message<'a>() -> $crate::Message<Self::Arguments<'a>> {
                Self::$first_tag
            }

            fn try_get_message<'a>(
                language: &$crate::locale::LanguageTag,
            ) -> Option<$crate::Message<Self::Arguments<'a>>> {
                if language == $first_tag {
                    return Some(Self::$first_tag);
                }

                $(if language == $tag {
                    return Some(Self::$tag)
                })*

                None
            }
        }
    };
}
