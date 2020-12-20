use cargo_toml::Manifest;
use color_eyre::eyre::{eyre, Result};

use async_std::path::{Path, PathBuf};

pub struct ApplicationDefinition {
  pub name: String,
  pub command: String,
  pub appdir_path: PathBuf,
}

impl ApplicationDefinition {
  pub fn new_from_guess() -> Result<Self> {
    let manifest = Manifest::from_path("Cargo.toml")?;
    let package = manifest
      .package
      .ok_or(eyre!("Manifest in Cargo.toml has no package section!"))?;

    let bin = manifest
      .bin
      .first()
      .ok_or(eyre!("No [[bin]] section found in Cargo.toml"))?;

    let command = bin
      .name
      .as_ref()
      .ok_or(eyre!("[[bin]] section has no 'name' key"))?;

    let name = package.name;
    let appdir_path_name = format!("{}.AppDir", name);
    let appdir_path = Path::new(&appdir_path_name).to_path_buf();
    let app_def = Self {
      name,
      command: String::from(command),
      appdir_path,
    };
    Ok(app_def)
  }
}
