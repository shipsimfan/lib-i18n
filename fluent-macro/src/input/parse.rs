use crate::IncludeFluentInput;
use proc_macro_util::{tokens::Literal, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for IncludeFluentInput {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        // Parse the path
        let path = parser.parse::<&'a Literal>()?;

        // Parse the options
        let options = parser.parse()?;

        // Verify the end of the macro
        if !parser.empty() {
            return Err(Error::new_at(
                "expected the end of the macro",
                parser.span(),
            ));
        }

        Ok(IncludeFluentInput {
            path: (path.to_string(), path.span()),
            options,
        })
    }
}
