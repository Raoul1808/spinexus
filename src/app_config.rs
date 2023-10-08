use confy::ConfyError;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub customs_path: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            customs_path: "".into(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfyError> {
        let cfg = confy::load("spinexus", None)?;
        Ok(cfg)
    }

    pub fn save(&self) -> Result<(), ConfyError> {
        confy::store("spinexus", None, self)
    }
}
