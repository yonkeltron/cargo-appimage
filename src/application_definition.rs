use cargo_toml::Manifest;
use color_eyre::eyre::{eyre, Result, WrapErr};
use paris::Logger;

use async_std::fs::OpenOptions;
use async_std::path::{Path, PathBuf};
use async_std::prelude::*;

use crate::app_image_metadata::{AppImageConfig, AppImageMetadata};

pub struct ApplicationDefinition {
  pub appdir_path: PathBuf,
  pub arch: String,
  pub command: String,
  pub name: String,
  pub icon: String,
}

impl ApplicationDefinition {
  pub async fn new_from_guess() -> Result<Self> {
    let manifest: Manifest<AppImageMetadata> =
      Manifest::from_path_with_metadata("Cargo.toml").wrap_err("Unable to load Cargo.toml")?;
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

    let rust_info = rust_info::get();
    let arch = rust_info
      .target_arch
      .unwrap_or_else(|| String::from("UNKNOWN_ARCH"));

    if package.metadata.is_none() {
      let config = AppImageConfig {
        icon: Some(icon.clone()),
        name: Some(name.clone()),
        target_architecture: Some(arch.clone()),
      };
      Self::append_config(config).await?;
    };

    let appdir_path_name = format!("{}.AppDir", &command);
    let appdir_path = Path::new(&appdir_path_name).to_path_buf();

    let app_def = Self {
      appdir_path,
      arch,
      command: String::from(command),
      icon,
      name,
    };
    Ok(app_def)
  }

  async fn append_config(config: AppImageConfig) -> Result<()> {
    let mut logger = Logger::new();
    logger.info("Adding config section to Cargo.toml");

    let mut cargo_toml_appender = OpenOptions::new().append(true).open("Cargo.toml").await?;
    let toml_section = toml::to_string_pretty(&config)?.bytes().collect::<Vec<_>>();
    cargo_toml_appender
      .write_all(b"\n[package.metadata.appimage]\n")
      .await?;
    cargo_toml_appender.write_all(&toml_section).await?;
    cargo_toml_appender.flush().await?;

    Ok(())
  }
}
