mod default;
mod display;
mod from;
mod get;
mod increment;
mod new;

/// The position of an element in a fluent file
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FluentPosition {
    /// The line the element is on
    line: u32,

    /// The column the element is at
    column: u32,
}
