use cargo_toml::Manifest;
use color_eyre::eyre::{eyre, Result};

use async_std::path::{Path, PathBuf};

pub struct ApplicationDefinition {
  pub appdir_path: PathBuf,
  pub arch: String,
  pub command: String,
  pub name: String,
}

impl ApplicationDefinition {
  pub fn new_from_guess() -> Result<Self> {
    let manifest = Manifest::from_path("Cargo.toml")?;
    let package = manifest
      .package
      .ok_or_else(|| eyre!("Manifest in Cargo.toml has no package section!"))?;

    let bin = manifest
      .bin
      .first()
      .ok_or_else(|| eyre!("No [[bin]] section found in Cargo.toml"))?;

    let command = bin
      .name
      .as_ref()
      .ok_or_else(|| eyre!("[[bin]] section has no 'name' key"))?;

    let name = package.name;
    let appdir_path_name = format!("{}.AppDir", &command);
    let appdir_path = Path::new(&appdir_path_name).to_path_buf();

    let rust_info = rust_info::get();
    let arch = rust_info
      .target_arch
      .unwrap_or_else(|| String::from("UNKNOWN_ARCH"));
    let app_def = Self {
      appdir_path,
      arch,
      command: String::from(command),
      name,
    };
    Ok(app_def)
  }
}
