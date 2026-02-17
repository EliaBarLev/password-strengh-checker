pub mod entropy;
pub mod patterns;
pub mod dictionary;
pub mod bruteforce;
pub mod leaked;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StrengthResult {
    pub score: u8,
    pub entropy_bits: f32,
    pub bruteforce_time: String,
    pub leaked: bool,
    pub reasons: Vec<String>,
}

pub fn analyze(password: &str) -> StrengthResult {
    let entropy_bits = entropy::calculate_entropy(password);
    let brute = bruteforce::estimate_time(entropy_bits);
    let leaked = leaked::is_leaked(password);
    let mut reasons = vec![];

    if patterns::has_sequences(password) {
        reasons.push("Keyboard sequence detected".into());
    }

    if dictionary::is_common(password) {
        reasons.push("Common dictionary password".into());
    }

    let score = ((entropy_bits / 1.5).min(100.0)) as u8;

    StrengthResult {
        score,
        entropy_bits,
        bruteforce_time: brute,
        leaked,
        reasons,
    }
}
