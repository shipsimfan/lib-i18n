use i18n::fluent::{
    FluentAttribute, FluentComment, FluentInlineText, FluentJunk, FluentMessage, FluentResource,
    FluentTerm,
};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentComment::new((1, 1), 1, "License Comment"));
    target.push(FluentComment::new((3, 1), 3, "Resource Comment"));
    target.push(FluentTerm::new(
        (5, 1),
        "brand-name",
        FluentInlineText::new((5, 15), "Aurora"),
        Vec::new(),
    ));
    target.push(FluentComment::new((7, 1), 2, "Group Comment"));
    target.push(FluentMessage::new(
        (9, 1),
        "key01",
        vec![FluentAttribute::new(
            (10, 5),
            "attr",
            FluentInlineText::new((10, 13), "Attribute"),
        )],
    ));
    target.push(FluentJunk::new(
        (12, 1),
        "ą=Invalid identifier\nć=Another one\n\n",
    ));
    target.push(FluentComment::new((15, 1), 1, "Message Comment"));
    target.push(FluentMessage::new_with(
        (16, 1),
        "key02",
        FluentInlineText::new((16, 9), "Value"),
        Vec::new(),
    ));
    target.push(FluentComment::new((18, 1), 1, "Standalone Comment"));
    target.push(FluentJunk::new(
        (19, 1),
        "    .attr = Dangling attribute\n\n",
    ));
    target.push(FluentComment::new(
        (21, 1),
        1,
        "There are 5 spaces on the line between key03 and key04.",
    ));
    target.push(FluentMessage::new_with(
        (22, 1),
        "key03",
        FluentInlineText::new((22, 9), "Value 03"),
        Vec::new(),
    ));
    target.push(FluentMessage::new_with(
        (24, 1),
        "key04",
        FluentInlineText::new((24, 9), "Value 04"),
        Vec::new(),
    ));
    target
}
