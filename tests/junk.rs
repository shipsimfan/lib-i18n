use i18n::fluent::{FluentJunk, FluentResource};

const SOURCE: &str = include_str!("junk.ftl");

#[test]
fn fluent_junk() {
    let target = create_target();

    let resource = i18n::fluent::parse(SOURCE);

    assert_eq!(target, resource);
}

fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentJunk::new(
        (1, 1),
        "## Two adjacent Junks.\n".to_string(),
    ));
    target.push(FluentJunk::new((2, 1), "err01 = {1x}\n".to_string()));
    target.push(FluentJunk::new((3, 1), "err02 = {2x}\n\n".to_string()));
    target.push(FluentJunk::new((5, 1), "# A single Junk.\n".to_string()));
    target.push(FluentJunk::new((6, 1), "err03 = {1x\n2\n\n".to_string()));
    target.push(FluentJunk::new(
        (9, 1),
        "# A single Junk.\ną=Invalid identifier\nć=Another one\n\n".to_string(),
    ));
    target.push(FluentJunk::new(
        (13, 1),
        "# The COMMENT ends this junk.\n".to_string(),
    ));
    target.push(FluentJunk::new((14, 1), "err04 = {\n".to_string()));
    target.push(FluentJunk::new((15, 1), "# COMMENT\n\n".to_string()));
    target.push(FluentJunk::new(
        (17, 1),
        "# The COMMENT ends this junk.\n".to_string(),
    ));
    target.push(FluentJunk::new(
        (18, 1),
        "# The closing brace is a separate Junk.\n".to_string(),
    ));
    target.push(FluentJunk::new((19, 1), "err04 = {\n".to_string()));
    target.push(FluentJunk::new((20, 1), "# COMMENT\n}".to_string()));
    target
}
