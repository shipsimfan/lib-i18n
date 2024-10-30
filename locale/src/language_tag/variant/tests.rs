use crate::Variant;

#[test]
fn len_test() {
    for i in 1..=Variant::MAX_LENGTH {
        do_len_test(i);
    }
}

fn do_len_test(len: usize) {
    const SOURCE: [u8; Variant::MAX_LENGTH] = *b"abcdefgh";

    let tag = &SOURCE[..len];

    let language = Variant::new(tag).unwrap();
    assert_eq!(language.len(), len);
    assert_eq!(language.as_slice(), tag);
}
