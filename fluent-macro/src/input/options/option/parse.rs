use super::IncludeFluentOption;
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
                Ok(IncludeFluentOption::Fallback {
                    value: fallback.to_string(),
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
