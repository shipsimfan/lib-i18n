use crate::IncludeFluentFormat;
use proc_macro_util::{
    to_tokens,
    tokens::{Identifier, Literal},
    Generator, ToTokens,
};

impl ToTokens for IncludeFluentFormat {
    fn to_tokens(self, generator: &mut Generator) {
        let IncludeFluentFormat { language, string } = self;

        to_tokens! { generator
            #language => { #string },
        }
    }
}
