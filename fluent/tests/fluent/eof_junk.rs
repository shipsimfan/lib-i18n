use i18n_fluent::{FluentComment, FluentJunk, FluentResource};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentComment::new(
        (1, 1),
        3,
        "NOTE: Disable final newline insertion when editing this file.",
    ));
    target.push(FluentJunk::new((3, 1), "000"));
    target
}
