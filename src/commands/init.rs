use color_eyre::eyre::{eyre, Result, WrapErr};
use paris::Logger;

use async_std::fs;
use async_std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

const SIXTY_FOUR_BIT_URL: &str = "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage";
const THIRTY_TWO_BIT_URL: &str = "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-i386.AppImage";

pub async fn execute() -> Result<()> {
  let rust_info = rust_info::get();
  let arch = rust_info
    .target_arch
    .unwrap_or_else(|| String::from("UNKNOWN_ARCH"));
  let mut logger = Logger::new();
  logger.log("Initializing AppImage Workspace");
  logger.info(format!("Detected target architecture: {}", arch));

  let download_url = url_for_arch(&arch)?;
  logger.loading(format!(
    "Downloading linuxdeploy AppImage from {}",
    &download_url
  ));

  let linuxdeploy_appimage_contents = download_linuxdeploy_appimage_contents(&download_url).await?;
  let output_path_name = format!("linuxdeploy-{}.AppImage", arch);
  let output_path = Path::new(&output_path_name).to_path_buf();
  fs::write(&output_path, &linuxdeploy_appimage_contents).await?;
  logger.info(format!(
    "Saved {} bytes to {}",
    linuxdeploy_appimage_contents.len(),
    output_path.display()
  ));

  make_executable(&output_path).await?;
  logger.info(format!(
    "Made saved AppImage executable at {}",
    output_path.display()
  ));

  setup_with_linuxdeploy(&output_path).await?;

  Ok(())
}

async fn download_linuxdeploy_appimage_contents(url: &str) -> Result<Vec<u8>> {
  let req = surf::get(url);
  let client = surf::client().with(surf::middleware::Redirect::new(5));
  match client.recv_bytes(req).await {
    Ok(linux_deploy_appimage_contents) => Ok(linux_deploy_appimage_contents),
    Err(reason) => Err(eyre!(
      "Unable to download linuxdeploy AppImage from {} because {}",
      url,
      reason
    )),
  }
}

fn url_for_arch(arch: &str) -> Result<String> {
  match arch {
    "x86_64" => Ok(String::from(SIXTY_FOUR_BIT_URL)),
    "i386" => Ok(String::from(THIRTY_TWO_BIT_URL)),
    _ => Err(eyre!(
      "Unrecognized or unsupported target architecture {}",
      arch
    )),
  }
}

async fn make_executable(path: &PathBuf) -> Result<()> {
  if path.is_file().await {
    let mut perm = fs::metadata(path).await?.permissions();
    perm.set_mode(0o744);
    fs::set_permissions(path, perm).await?;

    Ok(())
  } else {
    Err(eyre!("File at {} does not exist", path.display()))
  }
}

async fn setup_with_linuxdeploy(path: &PathBuf) -> Result<()> {
  let absolute_path = path.canonicalize().await?;
  let status = Command::new(absolute_path)
    .args(&["--appdir", "AppDir"])
    .status()
    .wrap_err_with(|| format!("Unable to spawn setup command from {}", path.display()))?;

  if status.success() {
    Ok(())
  } else {
    match status.code() {
      Some(code) => Err(eyre!("Setup command returned an error code of {}", code)),
      None => Err(eyre!(
        "Setup command failed and didn't provide an error code."
      )),
    }
  }
}
