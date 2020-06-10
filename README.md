
## Usage

```rust
// build.rs
// https://doc.rust-lang.org/cargo/reference/build-scripts.html

// for single proto
fn main() {
  println!("cargo:rerun-if-changed=./path/to/my_proto.proto");
  proto_code_gen::compile_protos("./path/to/my_proto.proto").unwrap();
}

// for multiple proto
fn main() {
  let list_of_protos = vec!["./path/to/my_proto.proto"];
  for &proto_path in &list_of_protos {
    println!("cargo:rerun-if-changed={}", &proto_path);
    proto_code_gen::compile_protos(proto_path).unwrap();
  }
}
```