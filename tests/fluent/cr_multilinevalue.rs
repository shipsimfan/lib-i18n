use i18n::fluent::{FluentInlineText, FluentMessage, FluentPattern, FluentResource};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentMessage::new(
        (1, 1),
        "key01",
        Some(FluentPattern::new(vec![FluentInlineText::new(
            (1, 8),
            "\r\r    Value 03\r    Continued\r\r    and continued\r    \r    and continued\r\r    .title = Title\r\r\r### This entire file uses CR as EOL.\r",
        )
        .into()])),
        Vec::new(),
    ));
    target
}
