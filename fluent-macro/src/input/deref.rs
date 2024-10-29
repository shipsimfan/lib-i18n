use crate::{IncludeFluentInput, IncludeFluentOptions};
use std::ops::Deref;

impl Deref for IncludeFluentInput {
    type Target = IncludeFluentOptions;

    fn deref(&self) -> &Self::Target {
        self.options()
    }
}

impl AsRef<IncludeFluentOptions> for IncludeFluentInput {
    fn as_ref(&self) -> &IncludeFluentOptions {
        &self.options
    }
}
