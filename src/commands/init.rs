use color_eyre::eyre::{eyre, Result};
use paris::Logger;

use async_std::fs;
use async_std::path::{Path, PathBuf};

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
  let output_path = Path::new(&output_path_name);
  fs::write(output_path, &linuxdeploy_appimage_contents).await?;
  logger.info(format!(
    "Saved {} bytes to {}",
    linuxdeploy_appimage_contents.len(),
    output_path.display()
  ));

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
