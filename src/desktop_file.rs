use askama::Template;
use color_eyre::eyre::Result;

use async_std::fs;
use async_std::path::PathBuf;

#[derive(Template)]
#[template(path = "desktop.txt")]
pub struct DesktopFile {
  name: String,
  command: String,
}

impl DesktopFile {
  pub fn new(name: &str, command: &str) -> Self {
    Self {
      name: String::from(name),
      command: String::from(command),
    }
  }

  pub async fn render_to_file(self, path: &PathBuf) -> Result<usize> {
    let contents = self.render()?;
    fs::write(path, &contents).await?;

    Ok(contents.len())
  }
}
