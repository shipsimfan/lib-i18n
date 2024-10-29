use crate::IncludeFluentInput;
use proc_macro_util::{tokens::Literal, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for IncludeFluentInput {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        // Parse the path
        let path = parser.parse::<&'a Literal>()?;
        let mut path_str = path.to_string();
        match match path_str.strip_prefix('"') {
            Some(missing_prefix) => missing_prefix,
            None => return Err(Error::new_at("expected a string", path.span())),
        }
        .strip_suffix('"')
        {
            Some(path) => path_str = path.to_string(),
            None => return Err(Error::new_at("expected a string", path.span())),
        };

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
            path: (path_str, path.span()),
            options,
        })
    }
}
