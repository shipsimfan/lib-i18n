use i18n::fluent::{FluentInlineText, FluentMessage, FluentResource};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentMessage::new_with(
        (1, 1),
        "key01",
        FluentInlineText::new(
            (1, 8),
            "\r\r    Value 03\r    Continued\r\r    and continued\r    \r    and continued\r\r    .title = Title\r\r\r### This entire file uses CR as EOL.\r",
        ),
        Vec::new(),
    ));
    target
}
