# Tests for Rust's ICU4X Unicode Collator

The ICU4X library allows one to reduce binary size by only including the required unicode data for the specified locales.  This is achieved by using the `BlobDataProvider` which allows one to embed the data directly into the binary.  The blob is generated using the `icu4x-datagen` tool.  The `--markers-for-bin` option lets one further reduce the binary size by analyzing the binary and only including the data that is actually required.

In this test, I include only the Greek locale data needed for sorting Greek.  I test sorting at the Primary, Secondary, Tertiary, and Quaternary levels.

## How to build

Install icu4x-datagen with (version should match version of library):  
cargo install icu4x-datagen --version 2.1.1 --force

Generate the minimal Greek collation data, run:  
icu4x-datagen --format blob --locales el --markers all --out greek_collation_blob.postcard  

Start with the above which includes all el data.  Then compile with this:  

cargo build --release && icu4x-datagen --markers-for-bin target/release/libicu4x-unicode-tests.rlib --locales el --format blob --out greek_collation_blob.postcard --overwrite

The --markers-for-bin option includes only the keys used in the binary.  So this may need to be run three times: first to create the binary, then to generate the data based on the binary, and a third time to embed this generated data inside the binary.

For the locale, just used "el" for icu4x-datagen. In the code, you can specify preferences such as "el-u-kn-true", which is used to sort numbers numerically rather than as strings.

## Reference 
https://www.credativ.de/en/blog/credativ-inside/icu4x-what-it-can-do-and-how-to-do-it/  
https://github.com/unicode-org/icu4x/blob/main/tutorials/data-management.md
