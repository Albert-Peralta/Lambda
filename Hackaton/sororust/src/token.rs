/// Convierte kilogramos de basura a tokens
/// 1 token = 15 kg de basura = $8 USD
pub fn kg_to_tokens(kg: f64) -> f64 {
    kg / 15.0
}

/// Convierte tokens a kilogramos
pub fn tokens_to_kg(tokens: f64) -> f64 {
    tokens * 15.0
}

/// Obtiene el valor en USD de los tokens
pub fn tokens_to_usd(tokens: f64) -> f64 {
    tokens * 8.0
}

/// Obtiene el valor en tokens de USD
pub fn usd_to_tokens(usd: f64) -> f64 {
    usd / 8.0
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub id: String,
    pub amount: f64,
    pub state: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Token {
    pub fn new(amount: f64, state: String) -> Self {
        Token {
            id: uuid::Uuid::new_v4().to_string(),
            amount,
            state,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn usd_value(&self) -> f64 {
        tokens_to_usd(self.amount)
    }

    pub fn kg_equivalent(&self) -> f64 {
        tokens_to_kg(self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kg_to_tokens() {
        assert_eq!(kg_to_tokens(15.0), 1.0);
        assert_eq!(kg_to_tokens(30.0), 2.0);
        assert_eq!(kg_to_tokens(7.5), 0.5);
    }

    #[test]
    fn test_tokens_to_usd() {
        assert_eq!(tokens_to_usd(1.0), 8.0);
        assert_eq!(tokens_to_usd(2.0), 16.0);
        assert_eq!(tokens_to_usd(0.5), 4.0);
    }

    #[test]
    fn test_usd_to_tokens() {
        assert_eq!(usd_to_tokens(8.0), 1.0);
        assert_eq!(usd_to_tokens(16.0), 2.0);
    }
}