use i18n::fluent::{FluentComment, FluentInlineText, FluentMessage, FluentPattern, FluentResource};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentComment::new(
        (1, 1),
        3,
        "NOTE: Disable final newline insertion when editing this file.",
    ));
    target.push(FluentMessage::new(
        (3, 1),
        "no-eol",
        Some(FluentPattern::new(vec![FluentInlineText::new(
            (3, 10),
            "No EOL",
        )
        .into()])),
        Vec::new(),
    ));
    target
}
