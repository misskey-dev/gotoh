use napi::bindgen_prelude::*;
use std::{io::Cursor, path::{Path, PathBuf}};
use zip;

pub struct ZipReader {}

#[napi(js_name = "ZipReader")]
pub struct JsZipReader {
  destination: DestinationType
}

enum DestinationType {
  Path(Box<Path>)
}

#[napi]
impl JsZipReader {
  #[napi]
  pub fn via_buffer(&self, buffer: Buffer) -> Result<()> {
    let mut zip = zip::ZipArchive::new(Cursor::new(buffer.as_ref())).or_else(|err| {
      Err(Error::new(
        Status::InvalidArg,
        format!("Failed to open zip archive: {}", err)
      ))
    })?;

    self.extract(&mut zip)
  }

  #[napi(factory)]
  pub fn with_destination_path(path: String) -> Self {
    Self {
      destination: DestinationType::Path(PathBuf::from(&path).into_boxed_path())
    }
  }

  fn extract(&self, zip: &mut zip::ZipArchive<Cursor<&[u8]>>) -> Result<()> {
    match &self.destination {
      DestinationType::Path(path) => {
        zip.extract(path).or_else(|err| {
          Err(Error::new(
            Status::InvalidArg,
            format!("Failed to extract zip archive: {}", err)
          ))
        })?;
      }
    }
    Ok(())
  }
}
