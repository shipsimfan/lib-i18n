use super::IncludeFluentFormatInsert;
use proc_macro_util::{to_tokens, ToTokens};

impl ToTokens for IncludeFluentFormatInsert {
    fn to_tokens(self, generator: &mut proc_macro_util::Generator) {
        let name = self.name;

        to_tokens! { generator
            , args.#name
        };
    }
}
