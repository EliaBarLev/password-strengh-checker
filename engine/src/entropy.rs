///
/// Simple Shannon entropy implementation.
///
pub fn calculate_entropy(password: &str) -> f32 {
    if password.is_empty() {
        return 0.0;
    }

    let mut counts = [0u32; 256];
    for &b in password.as_bytes() {
        counts[b as usize] += 1;
    }

    let mut entropy = 0.0;
    let len = password.len() as f32;

    for &count in &counts {
        if count > 0 {
            let p = count as f32 / len;
            entropy -= p * p.log2();
        }
    }

    entropy * len
}
