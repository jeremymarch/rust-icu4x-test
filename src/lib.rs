// To install the ICU4X data generator, run (version should match version of library):
// cargo install icu4x-datagen --version 2.1.1 --force
// To generate the Greek collation data, run:
// icu4x-datagen --format blob --locales el --markers all --out greek_collation.blob
//
use icu::locale::locale;
use icu_collator::options::CollatorOptions;
use icu_collator::Collator;
use icu_provider_blob::BlobDataProvider;

pub fn sort_words<'a>(
    words: &'a mut [&'a str],
) -> Result<Vec<&'a str>, Box<dyn std::error::Error>> {
    // Embed the minimal Greek-only collation data
    let blob_provider =
        BlobDataProvider::try_new_from_static_blob(include_bytes!("../greek_collation.blob"))
            .unwrap();

    let collator = Collator::try_new_with_buffer_provider(
        &blob_provider,
        locale!("el").into(),
        CollatorOptions::default(),
    )
    .expect("Greek collation data present");

    words.sort_by(|a, b| collator.as_borrowed().compare(a, b));

    Ok(words.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut words = vec!["ἄνθρωπος", "ἀγορά", "ἄγγελος", "Ἀθήνα", "ἀρετή"];
        let result = sort_words(&mut words);
        assert_eq!(
            result.unwrap(),
            vec!["ἄγγελος", "ἀγορά", "Ἀθήνα", "ἄνθρωπος", "ἀρετή"]
        );
    }
}
