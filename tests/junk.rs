const TARGET: &[((u32, u32), &str)] = &[
    ((1, 1), "## Two adjacent Junks.\n"),
    ((2, 1), "err01 = {1x}\n"),
    ((3, 1), "err02 = {2x}\n\n"),
    ((5, 1), "# A single Junk.\n"),
    ((6, 1), "err03 = {1x\n2\n\n"),
    (
        (9, 1),
        "# A single Junk.\ną=Invalid identifier\nć=Another one\n\n",
    ),
    ((13, 1), "# The COMMENT ends this junk.\n"),
    ((14, 1), "err04 = {\n"),
    ((15, 1), "# COMMENT\n\n"),
    ((17, 1), "# The COMMENT ends this junk.\n"),
    ((18, 1), "# The closing brace is a separate Junk.\n"),
    ((19, 1), "err04 = {\n"),
    ((20, 1), "# COMMENT\n}"),
];

const PATH: &str = "tests/junk.ftl";

#[test]
fn junk() {
    let resource = i18n::fluent::parse_file(PATH).unwrap();

    assert_eq!(resource.junk().len(), TARGET.len());
    for (junk, ((line, column), content)) in resource.junk().iter().zip(TARGET) {
        assert_eq!(junk.position().line(), *line);
        assert_eq!(junk.position().column(), *column);
        assert_eq!(junk.content(), *content);
    }
}
