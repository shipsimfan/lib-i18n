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
            ::i18n::translation::message_key!
        };

        let mut group = generator.group_parenthesis();
        let group = &mut group;

        to_tokens! { group
            pub #name
        };

        if variables.len() > 0 {
            to_tokens! { group
                <'a> {
                    #variables
                }
            };
        }

        to_tokens! { group
            [
                #format
            ]
        };

        to_tokens! { generator
            ;
        };
    }
}
