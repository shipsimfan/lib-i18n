use super::IncludeFluentFormatInsert;
use proc_macro_util::tokens::Identifier;

impl IncludeFluentFormatInsert {
    pub fn new<T: Into<Identifier>>(name: T) -> Self {
        IncludeFluentFormatInsert { name: name.into() }
    }
}
