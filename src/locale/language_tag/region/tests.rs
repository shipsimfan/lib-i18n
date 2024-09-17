use crate::locale::Region;

#[test]
fn len_test() {
    for i in Region::MIN_LENGTH..=Region::MAX_LENGTH {
        do_len_test(i);
    }
}

fn do_len_test(len: usize) {
    const SOURCE: [u8; Region::MAX_LENGTH] = *b"ABC";

    let tag = &SOURCE[..len];

    let region = Region::new(tag).unwrap();
    assert_eq!(region.len(), len);
    assert_eq!(region.as_slice(), tag);
}
