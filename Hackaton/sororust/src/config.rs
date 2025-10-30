pub const KG_PER_TOKEN: f64 = 15.0;
pub const USD_PER_TOKEN: f64 = 8.0;

pub const SUPPORTED_STATES: &[&str] = &["morelos", "morelia"];

pub fn validate_state(state: &str) -> bool {
    SUPPORTED_STATES.contains(&state.to_lowercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_state() {
        assert!(validate_state("morelos"));
        assert!(validate_state("MORELOS"));
        assert!(validate_state("morelia"));
        assert!(!validate_state("invalid"));
    }
}