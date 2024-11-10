use crate::IncludeFluentMessage;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for IncludeFluentMessage {
    fn to_tokens(self, generator: &mut Generator) {
        let IncludeFluentMessage {
            name,
            variables,
            format,
        } = self;

        to_tokens! { generator
            ::i18n::translation::message_key!(pub #name [
                #format
            ]);
        }
    }
}
