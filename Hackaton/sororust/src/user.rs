use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub state: String,
    pub tokens: f64,
    pub total_kg_collected: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn new(name: String, state: String) -> Self {
        User {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            state,
            tokens: 0.0,
            total_kg_collected: 0.0,
            created_at: Utc::now(),
        }
    }

    pub fn add_tokens(&mut self, amount: f64) {
        if amount > 0.0 {
            self.tokens += amount;
        }
    }

    pub fn remove_tokens(&mut self, amount: f64) -> bool {
        if self.tokens >= amount && amount > 0.0 {
            self.tokens -= amount;
            true
        } else {
            false
        }
    }

    pub fn add_trash(&mut self, kg: f64) {
        if kg > 0.0 {
            self.total_kg_collected += kg;
        }
    }

    pub fn get_usd_value(&self) -> f64 {
        self.tokens * 8.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("Juan".to_string(), "morelos".to_string());
        assert_eq!(user.name, "Juan");
        assert_eq!(user.state, "morelos");
        assert_eq!(user.tokens, 0.0);
    }

    #[test]
    fn test_add_tokens() {
        let mut user = User::new("Juan".to_string(), "morelos".to_string());
        user.add_tokens(5.0);
        assert_eq!(user.tokens, 5.0);
    }

    #[test]
    fn test_remove_tokens() {
        let mut user = User::new("Juan".to_string(), "morelos".to_string());
        user.add_tokens(10.0);
        assert!(user.remove_tokens(5.0));
        assert_eq!(user.tokens, 5.0);
        assert!(!user.remove_tokens(10.0));
    }
}