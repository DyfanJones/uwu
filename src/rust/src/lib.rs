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
/// Generate new Short UUID
///
/// @param n the number of short uuids to generate
/// @name short_uuid
/// @export
fn short_flickr_base58(n: i32) -> Strings {
    let n = n as usize;

    let range = 0..n;
    range
        .into_iter()
        .map(|_| ShortUuid::generate().to_string())
        .collect::<Strings>()
}

#[extendr]
/// @export
/// @rdname short_uuid
fn short_bitcoin58(n: i32) -> Strings {
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
fn uuid_flickr_to_short_(uuid: Strings) -> Strings {
    uuid.into_iter()
        .map(|u| ShortUuid::from_uuid_str(u).unwrap().to_string())
        .collect::<Strings>()
}

#[extendr]
fn uuid_bitcoin58_to_short_(uuid: Strings) -> Strings {
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

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod uwu;
    fn new_v4;
    fn new_v7;
    fn impute_uuid_;
    fn short_flickr_base58;
    fn short_bitcoin58;
    fn uuid_flickr_to_short_;
    fn uuid_bitcoin58_to_short_;
}
