use fluent::{
    FluentAttribute, FluentBlockText, FluentInlineText, FluentJunk, FluentMessage, FluentResource,
};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentMessage::new_with(
        (1, 1),
        "key01",
        FluentBlockText::new((4, 5), "Value"),
        Vec::new(),
    ));
    target.push(FluentMessage::new(
        (6, 1),
        "key02",
        vec![FluentAttribute::new(
            (9, 5),
            "attr",
            FluentInlineText::new((9, 13), "Attribute"),
        )],
    ));
    target.push(FluentMessage::new_with(
        (12, 1),
        "key03",
        vec![
            FluentBlockText::new((13, 5), "Value"),
            FluentBlockText::new((14, 5), "Continued"),
            FluentBlockText::new((17, 5), "Over multiple"),
            FluentBlockText::new((18, 5), "Lines"),
        ],
        vec![FluentAttribute::new(
            (22, 5),
            "attr",
            FluentInlineText::new((22, 13), "Attribute"),
        )],
    ));
    target.push(FluentMessage::new_with(
        (25, 1),
        "key05",
        FluentInlineText::new((25, 29), "Value"),
        Vec::new(),
    ));
    target.push(FluentJunk::new(
        (27, 1),
        "key06 = { 1 ->\n\n\n         [one] One\n\n\n\n\n        *[two] Two\n\n\n\n    }",
    ));
    target
}
