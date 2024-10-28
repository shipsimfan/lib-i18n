use crate::IncludeFluent;
use proc_macro_util::{tokens::Literal, Error, Parse, Parser, Result};

/// Actually parse the path from `parser`
fn parse_path<'a>(parser: &mut Parser<'a>) -> Result<String> {
    let path = parser.parse::<&'a Literal>()?.to_string();

    if !parser.empty() {
        return Err(Error::new_at("unexpected", parser.span()));
    }

    Ok(path)
}

impl<'a> Parse<'a> for IncludeFluent {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let path = parse_path(parser)?;

        Ok(IncludeFluent {})
    }
}
