use super::{Method, Service};
use crate::{generate_doc_comments, naive_snake_case};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generate boilerplate code for gRPC server
///
/// This takes some `Service` and will generate a `TokenStream` that contains
/// a public module with the generated client.
pub fn generate<T: Service>(service: &T, proto_path: &str) {
  generate_service_file(service, proto_path);
  generate_file_for_each_method(service, proto_path);

  // format generated files
  #[cfg(feature = "rustfmt")]
  {
    let source_dir = format!("{}/src", std::env::current_dir().unwrap().to_str().unwrap());
    super::fmt(&source_dir);
  }
}
fn generate_service_file<T: Service>(service: &T, proto_path: &str) {
  let stream = generate_service(service, proto_path);

  let code = format!("{}", stream);
  let filename = format!(
    "{}/src/service.rs",
    std::env::current_dir().unwrap().to_str().unwrap()
  );
  write_code_to_file(&code, &filename).expect("failed to write result to file");
}

fn write_code_to_file(
  //
  code: &str,
  filename: &str,
) -> std::io::Result<()> {
  use std::fs::File;
  use std::io::prelude::*;

  let mut file = File::create(filename)?;
  file.write_all(code.as_bytes())?; // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/create.html

  Ok(())
}

fn generate_service<T: Service>(service: &T, proto_path: &str) -> TokenStream {
  let service_name = quote::format_ident!("{}", service.name());
  let service_ident = quote::format_ident!("{}Service", service.name());
  let service_doc = generate_doc_comments(service.comment());
  let service_methods = generate_methods(service, proto_path);

  let stream = quote! {
    // Generated client implementations.
    //
    #service_doc
    #[derive(Default)]
    pub struct #service_ident {}

    #[tonic::async_trait]
    impl #service_name for #service_ident {
      #service_methods
    }
  };

  stream
}

fn generate_each_method_for_service<T: Method>(
  method: &T,
  proto_path: &str,
  path: String,
) -> TokenStream {
  let codec_name = syn::parse_str::<syn::Path>(T::CODEC_PATH).unwrap();
  let ident = format_ident!("{}", method.name());

  let (request, response) = method.request_response_name(proto_path);

  quote! {
    async fn #ident(
      &mut self,
      request: impl tonic::IntoRequest<#request>,
    ) -> Result<tonic::Response<#response>, tonic::Status> {
      action::#ident::handler(request).await
    }
  }
}

fn generate_methods<T: Service>(service: &T, proto_path: &str) -> TokenStream {
  let mut stream = TokenStream::new();

  for method in service.methods() {
    let path = format!(
      "/{}.{}/{}",
      service.package(),
      service.identifier(),
      method.identifier()
    );

    stream.extend(generate_doc_comments(method.comment()));

    let method = generate_each_method_for_service(method, proto_path, path);
    // let method = match (method.client_streaming(), method.server_streaming()) {
    //   (false, false) => generate_unary(method, proto_path, path),
    //   (false, true) => generate_server_streaming(method, proto_path, path),
    //   (true, false) => generate_client_streaming(method, proto_path, path),
    //   (true, true) => generate_streaming(method, proto_path, path),
    // };

    stream.extend(method);
  }

  stream
}

// ###########################################
// ############  for each method  ############
// ###########################################
fn generate_file_for_each_method<T: Service>(service: &T, proto_path: &str) {
  let mut stream = TokenStream::new();

  for method in service.methods() {
    let path = format!(
      "/{}.{}/{}",
      service.package(),
      service.identifier(),
      method.identifier()
    );

    stream.extend(generate_doc_comments(method.comment()));

    let method = generate_method_handler(method, proto_path, path);

    stream.extend(method);
  }
}

fn generate_method_handler<T: Method>(method: &T, proto_path: &str, path: String) -> TokenStream {
  let codec_name = syn::parse_str::<syn::Path>(T::CODEC_PATH).unwrap();
  let ident = format_ident!("{}", method.name());
  let (request, response) = method.request_response_name(proto_path);

  quote! {
    async fn #ident(
      &mut self,
      request: impl tonic::IntoRequest<#request>,
    ) -> Result<tonic::Response<#response>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()))
      })?;
      let codec = #codec_name::default();
      let path = http::uri::PathAndQuery::from_static(#path);
      self.inner.unary(request.into_request(), path, codec).await
    }
  }
}
