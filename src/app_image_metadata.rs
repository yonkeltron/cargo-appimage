use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppImageMetadata {
  pub appimage: Option<AppImageConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppImageConfig {
  pub name: Option<String>,
  pub icon: Option<String>,
  pub target_architecture: Option<String>,
}
