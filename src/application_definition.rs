use cargo_toml::Manifest;
use color_eyre::eyre::{eyre, Result, WrapErr};

use async_std::path::{Path, PathBuf};

use crate::app_image_metadata::AppImageMetadata;

pub struct ApplicationDefinition {
  pub appdir_path: PathBuf,
  pub arch: String,
  pub command: String,
  pub name: String,
  pub icon: String,
}

impl ApplicationDefinition {
  pub async fn new_from_guess() -> Result<Self> {
    let manifest: Manifest<AppImageMetadata> = Manifest::from_path_with_metadata("Cargo.toml")?;
    let package = manifest
      .package
      .ok_or_else(|| eyre!("Cargo.toml has no package section!"))?;

    let bin = manifest
      .bin
      .first()
      .ok_or_else(|| eyre!("No [[bin]] section found in Cargo.toml"))?;

    let command = bin
      .name
      .as_ref()
      .ok_or_else(|| eyre!("[[bin]] section has no 'name' key"))?;

    let name = {
      let pkg = package.clone();
      match pkg.metadata {
        Some(ref meta) => meta
          .clone()
          .appimage
          .map(|appimage| appimage.name)
          .map_or(pkg.name.clone(), |name| name.unwrap_or(pkg.name)),
        None => pkg.name,
      }
    };

    let icon = {
      let pkg = package.clone();
      match pkg.metadata {
        Some(ref meta) => meta
          .clone()
          .appimage
          .map(|appimage| appimage.icon)
          .map_or(pkg.name.clone(), |icon| icon.unwrap_or(pkg.name)),
        None => pkg.name,
      }
    };

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
      icon,
      name: String::from(name),
    };
    Ok(app_def)
  }
}
