use i18n::fluent::{FluentComment, FluentInlineText, FluentMessage, FluentResource};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentComment::new((1, 1), 1, "            ↓ BEL, U+0007"));
    target.push(FluentMessage::new_with(
        (2, 1),
        "control0",
        FluentInlineText::new((2, 12), "abcdef"),
        Vec::new(),
    ));
    target.push(FluentComment::new((4, 1), 1, "          ↓ DEL, U+007F"));
    target.push(FluentMessage::new_with(
        (5, 1),
        "delete",
        FluentInlineText::new((5, 10), "abcdef"),
        Vec::new(),
    ));
    target.push(FluentComment::new((7, 1), 1, "            ↓ BPM, U+0082"));
    target.push(FluentMessage::new_with(
        (8, 1),
        "control1",
        FluentInlineText::new((8, 12), "abcdef"),
        Vec::new(),
    ));
    target
}
