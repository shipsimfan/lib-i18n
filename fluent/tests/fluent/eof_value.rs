use fluent::{FluentComment, FluentInlineText, FluentMessage, FluentResource};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentComment::new(
        (1, 1),
        3,
        "NOTE: Disable final newline insertion when editing this file.",
    ));
    target.push(FluentMessage::new_with(
        (3, 1),
        "no-eol",
        FluentInlineText::new((3, 10), "No EOL"),
        Vec::new(),
    ));
    target
}
