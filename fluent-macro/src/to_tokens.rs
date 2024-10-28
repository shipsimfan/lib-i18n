use crate::IncludeFluent;
use proc_macro_util::{Generator, ToTokens};

impl ToTokens for IncludeFluent {
    fn to_tokens(self, generator: &mut Generator) {}
}
