// To install the ICU4X data generator, run (version should match version of library):
// cargo install icu4x-datagen --version 2.1.1 --force
// To generate the Greek collation data, run:
// icu4x-datagen --format blob --locales el --markers all --out greek_collation.blob
//
// https://github.com/unicode-org/icu4x/blob/main/tutorials/data-management.md
// cargo build --release && icu4x-datagen --markers-for-bin target/release/myapp --locales ja --format blob --out my_data_blob.postcard --overwrite
// cargo build --release && icu4x-datagen --markers-for-bin target/release/librust_icu4x_test.rlib --locales el --format blob --out greek_collation_blob.postcard --overwrite
//
use icu::locale::locale;
use icu_collator::options::CaseLevel;
use icu_collator::options::CollatorOptions;
use icu_collator::options::Strength;
use icu_collator::Collator;
use icu_provider_blob::BlobDataProvider;

pub fn sort_words<'a>(
    words: &'a mut [&'a str],
) -> Result<Vec<&'a str>, Box<dyn std::error::Error>> {
    // Embed the minimal Greek-only collation data
    // https://docs.rs/icu_provider_blob/latest/icu_provider_blob/struct.BlobDataProvider.html
    let blob_provider = BlobDataProvider::try_new_from_static_blob(include_bytes!(
        "../greek_collation_blob.postcard"
    ))
    .unwrap();

    let mut options = CollatorOptions::default();
    options.strength = Some(Strength::Secondary);
    //options.numeric = true;
    options.case_level = Some(CaseLevel::On);
    let collator =
        Collator::try_new_with_buffer_provider(&blob_provider, locale!("el").into(), options)
            .expect("Greek collation data present");

    words.sort_by(|a, b| collator.as_borrowed().compare(a, b));

    Ok(words.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut words = vec!["ἄνθρωπος", "ἀγορά", "ἄγγελος", "Ἀθήνα", "ἀρετή"];
        let result = sort_words(&mut words);
        assert!(result.is_ok());
        if let Ok(r) = result {
            assert_eq!(r, vec!["ἄγγελος", "ἀγορά", "Ἀθήνα", "ἄνθρωπος", "ἀρετή"]);
            assert_ne!(r, vec!["ἀγορά", "ἄγγελος", "Ἀθήνα", "ἄνθρωπος", "ἀρετή"]);
        }
    }
    /*
    #[test]
    fn it_works() {
        let mut words = vec!["α", "ᾱ", "ἄγγελος", "Ἀθήνα", "ἀρετή"];
        let result = sort_words(&mut words);
        assert!(result.is_ok());
        if let Ok(r) = result {
            assert_eq!(r, vec!["ἄγγελος", "ἀγορά", "Ἀθήνα", "ἄνθρωπος", "ἀρετή"]);
            assert_ne!(r, vec!["ἀγορά", "ἄγγελος", "Ἀθήνα", "ἄνθρωπος", "ἀρετή"]);
        }
    }
    */
}
