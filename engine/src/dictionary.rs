use phf::phf_set;

static COMMON_PASSWORDS: phf::Set<&'static str> = phf_set! {
    "password",
    "123456",
    "qwerty",
    "admin",
    "letmein",
};

pub fn is_common(password: &str) -> bool {
    COMMON_PASSWORDS.contains(&password.to_lowercase().as_str())
}
