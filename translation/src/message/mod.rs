/// A message in a paticular language
pub type Message<Arguments> = fn(&Arguments, &mut core::fmt::Formatter) -> core::fmt::Result;
