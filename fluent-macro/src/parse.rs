use crate::{merge::MergedModule, IncludeFluent, IncludeFluentInput, LoadedEntry};
use proc_macro_util::{Error, Parse, Parser, Result};

impl<'a> Parse<'a> for IncludeFluent {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let input = parser.parse::<IncludeFluentInput>()?;

        let path = std::env::current_dir().unwrap().join(input.path());
        let loaded = LoadedEntry::load(&path)?.ok_or(Error::new(format_args!(
            "invalid fluent file path \"{}\"",
            path.display()
        )))?;

        let merged = MergedModule::merge_entry(&loaded)?;

        IncludeFluent::render(input.fallback(), merged)
    }
}
