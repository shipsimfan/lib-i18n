use crate::Language;

#[test]
fn len_test() {
    for i in Language::MIN_LENGTH..=Language::MAX_LENGTH {
        do_len_test(i);
    }
}

fn do_len_test(len: usize) {
    const SOURCE: [u8; Language::MAX_LENGTH] = *b"abcdefgh";

    let tag = &SOURCE[..len];

    let language = Language::new(tag).unwrap();
    assert_eq!(language.len(), len);
    assert_eq!(language.as_slice(), tag);
}
