use crate::locale::{InvalidLanguageTag, Language, LanguageTag, Region, Script, Variant};
use std::borrow::Cow;

#[test]
fn en_us() {
    const INPUT: &[u8] = b"en_US";
    const TARGET: LanguageTag = LanguageTag {
        language: unsafe { Language::new_unchecked(b"en") },
        script: None,
        region: Region::new(b"US"),
        variants: Cow::Borrowed(&[]),
    };

    let output = LanguageTag::new(INPUT).unwrap();

    assert_eq!(output, TARGET);
    println!("{}", output);
}

#[test]
fn yue_hant_hk() {
    const INPUT: &[u8] = b"yue-Hant-HK";
    const TARGET: LanguageTag = LanguageTag {
        language: unsafe { Language::new_unchecked(b"yue") },
        script: Script::new(b"Hant"),
        region: Region::new(b"HK"),
        variants: Cow::Borrowed(&[]),
    };

    let output = LanguageTag::new(INPUT).unwrap();

    assert_eq!(output, TARGET);
    println!("{}", output);
}

#[test]
fn sr_cyrl() {
    const INPUT: &[u8] = b"sr-Cyrl";
    const TARGET: LanguageTag = LanguageTag {
        language: unsafe { Language::new_unchecked(b"sr") },
        script: Script::new(b"Cyrl"),
        region: None,
        variants: Cow::Borrowed(&[]),
    };

    let output = LanguageTag::new(INPUT).unwrap();

    assert_eq!(output, TARGET);
    println!("{}", output);
}

#[test]
fn rm_sursilv() {
    const INPUT: &[u8] = b"rm-sursilv";
    const TARGET: LanguageTag = LanguageTag {
        language: unsafe { Language::new_unchecked(b"rm") },
        script: None,
        region: None,
        variants: Cow::Borrowed(&[unsafe { Variant::new_unchecked(b"sursilv") }]),
    };

    let output = LanguageTag::new(INPUT).unwrap();

    assert_eq!(output, TARGET);
    println!("{}", output);
}

#[test]
fn empty() {
    const INPUT: &[u8] = b"";

    assert_eq!(
        LanguageTag::new(INPUT),
        Err(InvalidLanguageTag::InvalidLanguage)
    );
}

#[test]
fn on_char_language() {
    const INPUT: &[u8] = b"a";

    assert_eq!(
        LanguageTag::new(INPUT),
        Err(InvalidLanguageTag::InvalidLanguage)
    );
}

#[test]
fn trailing_dash() {
    const INPUT: &[u8] = b"ab-";

    assert_eq!(
        LanguageTag::new(INPUT),
        Err(InvalidLanguageTag::InvalidVariant)
    );
}

#[test]
fn invalid_character() {
    const INPUT: &[u8] = b"ab Test";

    assert_eq!(
        LanguageTag::new(INPUT),
        Err(InvalidLanguageTag::InvalidLanguage)
    );
}
