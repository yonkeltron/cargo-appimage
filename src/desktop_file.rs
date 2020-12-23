use color_eyre::eyre::{Result, WrapErr};
use liquid::ParserBuilder;
use serde::Serialize;
use tempfile::tempfile;

use async_std::path::PathBuf;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
pub struct DesktopFile {
  name: String,
  command: String,
  icon: String,
}

impl DesktopFile {
  pub fn new(name: &str, command: &str, icon: &str) -> Self {
    Self {
      name: String::from(name),
      command: String::from(command),
      icon: String::from(icon),
    }
  }

  pub fn render_from_template(self, template_path: &PathBuf) -> Result<String> {
    let template = ParserBuilder::with_stdlib()
      .build()?
      .parse_file(template_path)?;
    let vars = liquid::to_object(&self)?;

    let contents = template.render(&vars).wrap_err_with(|| {
      format!(
        "Error parsing desktop file template from {}",
        template_path.display()
      )
    })?;

    Ok(contents)
  }

  pub fn render_template_to_tempfile(self, template_file_path: &PathBuf) -> Result<File> {
    let contents = self.render_from_template(template_file_path)?;
    let mut dest_file = tempfile()?;
    let contents_bytes = contents.bytes().collect::<Vec<_>>();
    dest_file.write_all(&contents_bytes)?;

    Ok(dest_file)
  }
}
