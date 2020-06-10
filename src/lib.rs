use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream};
use quote::TokenStreamExt;

/// Prost generator
#[cfg(feature = "prost")]
#[cfg_attr(docsrs, doc(cfg(feature = "prost")))]
mod prost;

#[cfg(feature = "prost")]
#[cfg_attr(docsrs, doc(cfg(feature = "prost")))]
pub use prost::{compile_protos, configure};

#[cfg(feature = "rustfmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "rustfmt")))]
use std::io::{self, Write};
#[cfg(feature = "rustfmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "rustfmt")))]
use std::process::{exit, Command};

// pub trait Method: tonic_build::Method {}
pub trait Service {
  /// Path to the codec.
  const CODEC_PATH: &'static str;

  /// Comment type.
  type Comment: AsRef<str>;

  /// Method type.
  type Method: Method;

  /// Name of service.
  fn name(&self) -> &str;
  /// Package name of service.
  fn package(&self) -> &str;
  /// Identifier used to generate type name.
  fn identifier(&self) -> &str;
  /// Methods provided by service.
  fn methods(&self) -> &[Self::Method];
  /// Get comments about this item.
  fn comment(&self) -> &[Self::Comment];
}

/// Method generation trait.
///
/// Each service contains a set of generic
/// `Methods`'s that will be used by codegen
/// to generate abstraction implementations for
/// the provided methods.
pub trait Method {
  /// Path to the codec.
  const CODEC_PATH: &'static str;
  /// Comment type.
  type Comment: AsRef<str>;

  /// Name of method.
  fn name(&self) -> &str;
  /// Identifier used to generate type name.
  fn identifier(&self) -> &str;
  /// Method is streamed by client.
  fn client_streaming(&self) -> bool;
  /// Method is streamed by server.
  fn server_streaming(&self) -> bool;
  /// Get comments about this item.
  fn comment(&self) -> &[Self::Comment];
  /// Type name of request and response.
  fn request_response_name(&self, proto_path: &str) -> (TokenStream, TokenStream);
}

/// Format files under the out_dir with rustfmt
#[cfg(feature = "rustfmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "rustfmt")))]
pub fn fmt(out_dir: &str) {
  let dir = std::fs::read_dir(out_dir).unwrap();

  for entry in dir {
    let file = entry.unwrap().file_name().into_string().unwrap();
    if !file.ends_with(".rs") {
      continue;
    }
    let result = Command::new("rustfmt")
      .arg("--emit")
      .arg("files")
      .arg("--edition")
      .arg("2018")
      .arg(format!("{}/{}", out_dir, file))
      .output();

    match result {
      Err(e) => {
        eprintln!("error running rustfmt: {:?}", e);
        exit(1)
      }
      Ok(output) => {
        if !output.status.success() {
          io::stderr().write_all(&output.stderr).unwrap();
          exit(output.status.code().unwrap_or(1))
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
