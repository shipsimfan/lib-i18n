use crate::{IncludeFluent, IncludeFluentInput};
use proc_macro_util::{Parse, Parser, Result};

impl<'a> Parse<'a> for IncludeFluent {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let input = parser.parse::<IncludeFluentInput>()?;

        Ok(IncludeFluent {})
    }
}
