///
/// Very simplified brute-force time estimator.
/// Assumes:
/// - 1e9 guesses/sec (~1 GPU)
///
pub fn estimate_time(entropy_bits: f32) -> String {
    let guesses = 2_f64.powf(entropy_bits as f64);
    let gpus = 1e9_f64; // guesses per second

    let seconds = guesses / gpus;

    if seconds < 60.0 {
        return format!("{:.2} seconds", seconds);
    }
    if seconds < 3600.0 {
        return format!("{:.2} minutes", seconds / 60.0);
    }
    if seconds < 86400.0 {
        return format!("{:.2} hours", seconds / 3600.0);
    }

    format!("{:.2} days", seconds / 86400.0)
}