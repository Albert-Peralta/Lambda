use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateType {
    Morelos,
    Morelia,
}

impl StateType {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "morelos" => Some(StateType::Morelos),
            "morelia" => Some(StateType::Morelia),
            _ => None,
        }
    }

    pub fn animal_name(&self) -> &str {
        match self {
            StateType::Morelos => "Salmón",
            StateType::Morelia => "Tortuga",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            StateType::Morelos => "Estado de Morelos - Simbología: Salmón",
            StateType::Morelia => "Ciudad de Morelia - Simbología: Tortuga",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateConfig {
    pub state: StateType,
    pub animal_image_path: String,
}

impl StateConfig {
    pub fn new(state: StateType) -> Self {
        let animal_image_path = match state {
            StateType::Morelos => "assets/morelos.txt".to_string(),
            StateType::Morelia => "assets/morelia.txt".to_string(),
        };

        StateConfig {
            state,
            animal_image_path,
        }
    }

    pub fn get_animal_name(&self) -> &str {
        self.state.animal_name()
    }

    pub fn get_description(&self) -> &str {
        self.state.description()
    }
}