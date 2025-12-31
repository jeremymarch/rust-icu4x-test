use icu::locale::locale;
use icu_collator::Collator;
use icu_collator::options::CollatorOptions;
use icu_provider_blob::BlobDataProvider;

pub fn sort_words<'a>(
    words: &'a mut [&'a str],
    options: &CollatorOptions,
) -> Result<Vec<&'a str>, Box<dyn std::error::Error>> {
    // Embed the minimal Greek-only collation data
    // https://docs.rs/icu_provider_blob/latest/icu_provider_blob/struct.BlobDataProvider.html
    let blob_provider = BlobDataProvider::try_new_from_static_blob(include_bytes!(
        "../greek_collation_blob.postcard"
    ))
    .unwrap();

    let collator = Collator::try_new_with_buffer_provider(
        &blob_provider,
        locale!("el-u-kn-true").into(), //kn-true means to sort numbers numerically rather than as strings
        *options,
    )
    .expect("Greek collation data present");

    words.sort_by(|a, b| collator.as_borrowed().compare(a, b));
    /*
    for word in &mut *words {
        println!("{}", word);
    }
    */
    Ok(words.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_collator::options::CaseLevel;
    use icu_collator::options::Strength;

    const TEST_WORDS: [&str; 18] = [
        "α1",
        "α2",
        "α10",
        "α",
        "\u{03B1}\u{0304}", //apha with combing macron
        "ἃ ἅ",
        "ἀ",
        "ἄ",
        "Ά",
        "ἄγγελος",
        "\u{1FB1}", //apha with precomposed macron
        "ᾱ̓́ͅ",
        "ἀάατος",
        "ᾱ̓́ͅσομαι",
        "Ἀθήνα",
        "Α",
        "ἀρετή",
        "ά",
    ];

    #[test]
    fn test_primary() {
        let mut options = CollatorOptions::default();
        options.strength = Some(Strength::Primary);
        options.case_level = Some(CaseLevel::On); //whether to distinguish case above the tertiary level

        let mut words = TEST_WORDS.to_vec();
        let expected = vec![
            "α",
            "\u{03B1}\u{0304}", //apha with composing macron
            "ἀ",
            "ἄ",
            "\u{1FB1}", //apha with precomposed macron
            "ᾱ̓́ͅ",
            "ά",
            "Ά",
            "Α",
            "ἃ ἅ",
            "α1",
            "α2",
            "α10",
            "ἀάατος",
            "ἄγγελος",
            "Ἀθήνα",
            "ἀρετή",
            "ᾱ̓́ͅσομαι",
        ];
        let result = sort_words(&mut words, &options);
        assert!(result.is_ok());
        if let Ok(r) = result {
            assert_eq!(r, expected);
        }
    }

    #[test]
    fn test_secondary() {
        let mut options = CollatorOptions::default();
        options.strength = Some(Strength::Secondary);
        options.case_level = Some(CaseLevel::On); //whether to distinguish case above the tertiary level

        let mut words = TEST_WORDS.to_vec();
        let expected = vec![
            "α",
            "Α",
            "ἀ",
            "ἄ",
            "ά",
            "Ά",
            "\u{03B1}\u{0304}", //apha with composing macron
            "\u{1FB1}",         //apha with precomposed macron
            "ᾱ̓́ͅ",
            "ἃ ἅ",
            "α1",
            "α2",
            "α10",
            "ἀάατος",
            "ἄγγελος",
            "Ἀθήνα",
            "ἀρετή",
            "ᾱ̓́ͅσομαι",
        ];
        let result = sort_words(&mut words, &options);
        assert!(result.is_ok());
        if let Ok(r) = result {
            assert_eq!(r, expected);
        }
    }

    #[test]
    fn test_teriary() {
        let mut options = CollatorOptions::default();
        options.strength = Some(Strength::Tertiary);
        options.case_level = Some(CaseLevel::Off); //whether to distinguish case above the tertiary level

        let mut words = TEST_WORDS.to_vec();
        let expected = vec![
            "α",
            "Α",
            "ἀ",
            "ἄ",
            "ά",
            "Ά",
            "\u{03B1}\u{0304}", //apha with composing macron
            "\u{1FB1}",         //apha with precomposed macron
            "ᾱ̓́ͅ",
            "ἃ ἅ",
            "α1",
            "α2",
            "α10",
            "ἀάατος",
            "ἄγγελος",
            "Ἀθήνα",
            "ἀρετή",
            "ᾱ̓́ͅσομαι",
        ];
        let result = sort_words(&mut words, &options);
        assert!(result.is_ok());
        if let Ok(r) = result {
            assert_eq!(r, expected);
        }
    }

    #[test]
    fn test_quaternary() {
        let mut options = CollatorOptions::default();
        options.strength = Some(Strength::Quaternary);
        options.case_level = Some(CaseLevel::Off); //whether to distinguish case above the tertiary level

        let mut words = TEST_WORDS.to_vec();
        let expected = vec![
            "α",
            "Α",
            "ἀ",
            "ἄ",
            "ά",
            "Ά",
            "\u{03B1}\u{0304}", //apha with composing macron
            "\u{1FB1}",         //apha with precomposed macron
            "ᾱ̓́ͅ",
            "ἃ ἅ",
            "α1",
            "α2",
            "α10",
            "ἀάατος",
            "ἄγγελος",
            "Ἀθήνα",
            "ἀρετή",
            "ᾱ̓́ͅσομαι",
        ];
        let result = sort_words(&mut words, &options);
        assert!(result.is_ok());
        if let Ok(r) = result {
            assert_eq!(r, expected);
        }
    }
}
