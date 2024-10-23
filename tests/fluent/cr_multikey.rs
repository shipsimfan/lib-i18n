use i18n::fluent::{FluentInlineText, FluentMessage, FluentPattern, FluentResource};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentMessage::new(
        (1, 1),
        "key01",
        Some(FluentPattern::new(vec![FluentInlineText::new(
            (1, 9),
            "Value 01\rerr02 = Value 02\r\r\r### This entire file uses CR as EOL.\r",
        )
        .into()])),
        Vec::new(),
    ));
    target
}
