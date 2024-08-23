use extendr_api::prelude::*;
use short_uuid::{CustomTranslator, ShortUuid, ShortUuidCustom};
use uuid::Uuid;

#[extendr]
/// Generate new UUIDs
///
/// Note that if creating a v7 UUID, `uuid::UUIDgenerate(use.time = TRUE, output = "string")` is faster whereas a v4 UUID is ~20x faster using this implementation.
/// @param n the number of uuids to generate
/// @export
fn new_v4(n: i32) -> Strings {
    let n = n as usize;

    let range = 0..n;
    range
        .into_iter()
        .map(|_| Uuid::new_v4().hyphenated().to_string())
        .collect::<Strings>()
}

#[extendr]
/// @export
/// @rdname new_v4
fn new_v7(n: i32) -> Strings {
    let n = n as usize;

    let range = 0..n;
    range
        .into_iter()
        .map(|_| Uuid::now_v7().hyphenated().to_string())
        .collect::<Strings>()
}

#[extendr]
fn impute_uuid_(x: Strings, prefix: &str) {
    let mut x = x;

    for i in 0..x.len() {
        let xi = &x[i];
        if xi.is_na() {
            let uuid = Uuid::new_v4().hyphenated().to_string();
            x.set_elt(i, Rstr::from(format!("{prefix}{uuid}")))
        }
    }
}

#[extendr]
fn short_flickr_base58_(n: i32) -> Strings {
    let n = n as usize;

    let range = 0..n;
    range
        .into_iter()
        .map(|_| ShortUuid::generate().to_string())
        .collect::<Strings>()
}

#[extendr]
fn short_bitcoin58_(n: i32) -> Strings {
    let n = n as usize;
    let custom_alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let translator = CustomTranslator::new(custom_alphabet).unwrap();

    let range = 0..n;
    range
        .into_iter()
        .map(|_| ShortUuidCustom::generate(&translator).to_string())
        .collect::<Strings>()
}

#[extendr]
fn uuid_to_short_flickr_(uuid: Strings) -> Strings {
    uuid.into_iter()
        .map(|u| ShortUuid::from_uuid_str(u).unwrap().to_string())
        .collect::<Strings>()
}

#[extendr]
fn uuid_to_short_bitcoin58_(uuid: Strings) -> Strings {
    let custom_alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let translator = CustomTranslator::new(custom_alphabet).unwrap();

    uuid.into_iter()
        .map(|u| {
            ShortUuidCustom::from_uuid_str(u, &translator)
                .unwrap()
                .to_string()
        })
        .collect::<Strings>()
}

#[extendr]
fn short_flickr_to_uuid_(short_uuid: Strings) -> Strings {
    short_uuid
        .into_iter()
        .map(|u| {
            let uuid = ShortUuid::parse_str(u).unwrap().to_uuid();
            uuid.to_string()
        })
        .collect::<Strings>()
}

#[extendr]
fn short_bitcoin58_to_uuid_(short_uuid: Strings) -> Strings {
    let custom_alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let translator = CustomTranslator::new(custom_alphabet).unwrap();

    short_uuid
        .into_iter()
        .map(|u| {
            let uuid = ShortUuidCustom::parse_str(u, &translator)
                .unwrap()
                .to_uuid(&translator);
            uuid.unwrap().to_string()
        })
        .collect::<Strings>()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod uwu;
    fn new_v4;
    fn new_v7;
    fn impute_uuid_;
    fn short_flickr_base58_;
    fn short_bitcoin58_;
    fn uuid_to_short_flickr_;
    fn uuid_to_short_bitcoin58_;
    fn short_flickr_to_uuid_;
    fn short_bitcoin58_to_uuid_;
}
