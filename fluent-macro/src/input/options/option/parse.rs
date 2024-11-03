use super::IncludeFluentOption;
use i18n_locale::LanguageTag;
use proc_macro_util::{
    tokens::{Identifier, Literal},
    Error, Parse, Parser, Result, Token,
};

impl<'a> Parse<'a> for IncludeFluentOption {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let name = parser.parse::<&'a Identifier>()?;

        match name.to_string().as_str() {
            "fallback" => {
                parser.parse::<Token![=]>()?;
                let fallback = parser.parse::<&'a Literal>()?;
                let mut fallback_str = fallback.to_string();

                match match fallback_str.strip_prefix('"') {
                    Some(missing_prefix) => missing_prefix,
                    None => return Err(Error::new_at("expected a string", fallback.span())),
                }
                .strip_suffix('"')
                {
                    Some(fallback) => fallback_str = fallback.to_string(),
                    None => return Err(Error::new_at("expected a string", fallback.span())),
                };

                let value = LanguageTag::new(fallback_str.as_bytes()).map_err(|error| {
                    Error::new_at(
                        format_args!("invalid fallback language \"{}\" - {}", fallback_str, error),
                        fallback.span(),
                    )
                })?;

                Ok(IncludeFluentOption::Fallback {
                    value,
                    value_span: fallback.span(),
                    name_span: name.span(),
                })
            }
            name_str => Err(Error::new_at(
                format_args!("unknown option \"{}\"", name_str),
                name.span(),
            )),
        }
    }
}
