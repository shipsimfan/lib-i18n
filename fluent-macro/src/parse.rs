use crate::{merge::MergedModule, IncludeFluent, IncludeFluentInput, LoadedEntry};
use proc_macro_util::{Error, Parse, Parser, Result};

impl<'a> Parse<'a> for IncludeFluent {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let input = parser.parse::<IncludeFluentInput>()?;

        let loaded = LoadedEntry::load(std::env::current_dir().unwrap().join(input.path()))?
            .ok_or(Error::new("invalid path"))?;

        let merged = MergedModule::merge_entry(&loaded)?;

        IncludeFluent::render(input.fallback(), merged)
    }
}
