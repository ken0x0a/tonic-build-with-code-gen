#![allow(dead_code)]

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

const GENERATED_CODE_HEADER: &'static str = "// This file is generated using \"proto-code-gen\".
// DO NOT EDIT BY HAND!!
";

///
/// ## Args
/// - code: TokenStream to generate file
/// - pathname: relative path from `src`
pub fn write_stream_to_file(code: TokenStream, pathname: &str) -> std::io::Result<()> {
  use std::fs::File;
  use std::io::prelude::*;

  let filename = get_abs_path_from_root(pathname);

  let mut file = File::create(filename)?;
  file.write_all(format!("{}", code).as_bytes())?; // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/create.html

  Ok(())
}

/// write_code_to_file_with_header(code, GENERATED_CODE_HEADER, pathname);
pub fn write_stream_to_file_with_header_if_not_exist(
  code: TokenStream,
  pathname: &str,
  header: Option<&str>,
) -> std::io::Result<()> {
  let filename = get_abs_path_from_root(pathname);
  if !std::path::Path::new(&filename).exists() {
    write_stream_to_file_with_header(code, pathname, header)
  } else {
    Ok(())
  }
}
pub fn write_stream_to_file_with_header(
  code: TokenStream,
  pathname: &str,
  header: Option<&str>,
) -> std::io::Result<()> {
  use std::fs::File;
  use std::io::prelude::*;

  let filename = get_abs_path_from_root(pathname);

  let mut file = File::create(filename)?;
  file.write_all(format!("{}{}", header.unwrap_or(GENERATED_CODE_HEADER), code).as_bytes())?; // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/create.html

  Ok(())
}

/// ```
/// let filename = get_abs_path_from_root(pathname); // src/rust.rs => <project_dir>/src/rust.rs
/// ```
///
pub fn get_abs_path_from_root(pathname: &str) -> String {
  format!(
    "{}/{}",
    std::env::current_dir().unwrap().to_str().unwrap(),
    pathname
  )
}
