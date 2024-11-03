use crate::IncludeFluentModule;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for IncludeFluentModule {
    fn to_tokens(self, generator: &mut Generator) {
        let IncludeFluentModule {
            depth,
            sub_modules,
            messages,
        } = self;

        if messages.len() > 0 {
            to_tokens! { generator
                use
            };

            for _ in 0..depth {
                to_tokens! { generator
                    super::
                };
            }

            to_tokens! { generator
                supported_languages::*;
            };
        }

        for (name, module) in sub_modules {
            to_tokens! { generator
                pub mod #name {
                    #module
                }
            };
        }

        for message in messages {
            message.to_tokens(generator);
        }
    }
}
