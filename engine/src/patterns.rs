use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SEQ: Regex = Regex::new(r"(1234|abcd|qwer|asdf)").unwrap();
}

pub fn has_sequences(pw: &str) -> bool {
    SEQ.is_match(&pw.to_lowercase())
}
