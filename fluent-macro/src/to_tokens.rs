use crate::IncludeFluent;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for IncludeFluent {
    fn to_tokens(self, generator: &mut Generator) {
        let IncludeFluent {
            root,
            supported_languages,
        } = self;

        if supported_languages.len() > 0 {
            to_tokens! { generator
                pub mod supported_languages {
                    #supported_languages
                }
            };
        }

        root.to_tokens(generator);
    }
}
