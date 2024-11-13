use crate::IncludeFluentVariable;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for IncludeFluentVariable {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            IncludeFluentVariable::Display => {
                to_tokens! { generator
                    &'a dyn ::core::fmt::Display,
                };
            }
        }
    }
}
