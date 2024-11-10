use crate::IncludeFluentFormat;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for IncludeFluentFormat {
    fn to_tokens(self, generator: &mut Generator) {
        let IncludeFluentFormat {
            language,
            string,
            inserts,
        } = self;

        to_tokens! { generator
            #language => { |args, f| ::std::write!(f, #string #inserts) },
        }
    }
}
