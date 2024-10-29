use super::IncludeFluentOption;
use crate::IncludeFluentOptions;
use proc_macro_util::{Error, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for IncludeFluentOptions {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut fallback = None;

        while parser.step_parse::<Token![,]>().is_ok() {
            let option = parser.parse::<IncludeFluentOption>()?;

            match option {
                IncludeFluentOption::Fallback {
                    value: new_fallback,
                    value_span,
                    name_span,
                } => match fallback {
                    Some(_) => {
                        return Err(Error::new_at("multiple fallbacks specified", name_span));
                    }
                    None => fallback = Some((new_fallback, value_span)),
                },
            }
        }

        Ok(IncludeFluentOptions { fallback })
    }
}
