use crate::SupportedLanguage;
use i18n_locale::LanguageTag;
use proc_macro_util::{to_tokens, tokens::Literal, Generator, ToTokens};

impl ToTokens for SupportedLanguage {
    fn to_tokens(self, generator: &mut Generator) {
        let SupportedLanguage {
            identifier,
            tag:
                LanguageTag {
                    language,
                    script,
                    region,
                    variants,
                },
        } = self;

        to_tokens! { generator
            pub const #identifier: ::i18n::locale::LanguageTag<'static> = ::i18n::locale::LanguageTag
        };

        let mut group = generator.group_brace();
        let group = &mut group;

        let language = Literal::new(language.as_str());
        to_tokens! { group
            language: unsafe { ::i18n::locale::Language::new_unchecked(#language.as_bytes()) },
        };

        match script {
            Some(script) => {
                let script = Literal::new(script.as_str());

                to_tokens! { group
                    script: Some(unsafe { ::i18n::locale::Script::new_unchecked(#script.as_bytes()) }),
                };
            }
            None => {
                to_tokens! { group
                    script: None,
                };
            }
        }

        match region {
            Some(region) => {
                let region = Literal::new(region.as_str());

                to_tokens! { group
                    region: Some(unsafe { ::i18n::locale::Region::new_unchecked(#region.as_bytes()) }),
                };
            }
            None => {
                to_tokens! { group
                    region: None,
                };
            }
        }

        to_tokens! { group
            variants:
        };
        let mut variant_group = group.group_bracket();
        let variant_group = &mut variant_group;
        for variant in variants.iter() {
            let variant = Literal::new(variant.as_str());

            to_tokens! { variant_group
                unsafe { ::i18n::locale::Variant::new_unchecked(#variant.as_bytes()) },
            }
        }
        to_tokens! { group ,};

        to_tokens! { generator ; };
    }
}
