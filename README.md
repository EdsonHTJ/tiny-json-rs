
# tiny-json-rs

A minimalistic Rust library for encoding and decoding JSON, offering a lower memory footprint alternative to serde.

## Introduction

`tiny-json-rs` is an open-source library designed for efficiently handling JSON in Rust applications. It aims to provide functionality similar to the serde library but focuses on reducing memory usage, making it an ideal choice for resource-constrained environments or applications where performance is critical.

## Features

- **Efficient JSON Encoding/Decoding**: Optimized for lower memory usage.
- **Easy Integration**: Simple API, compatible with standard Rust structs.
- **Customizable**: Extendable to support various data types.

## Usage

Add `tiny-json-rs` to your Cargo.toml:

```toml
[dependencies]
tiny-json-rs = "0.2.5"
```

### Basic Examples

#### Struct Definition

Define your structs and derive `Deserialize` and `Serialize`:

```rust
use tiny_json_rs::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct A {
    pub a: i32,
    pub b: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct B {
    pub a: i32,
    pub b: Vec<String>,
}

```

Can also derive custom names

```rust
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct C {
    #[Rename = "CustomName"]
    pub a: i32,
    pub b: String,
}
```

#### Deserializing JSON

Convert a JSON string to a Rust struct:

```rust
let json = r#"{"a": 1, "b": "Hello"}"#;
let parsed: A = tiny_json_rs::decode(json.to_string()).unwrap();
assert_eq!(parsed.a, 1);
assert_eq!(parsed.b, "Hello");
```

#### Serializing to JSON

Convert a Rust struct to a JSON string:

```rust
let obj = C {
    a: 1,
    b: "Hello".to_string(),
};

let json = tiny_json_rs::encode(obj);
assert_eq!(json, r#"{"CustomName":1,"b":"Hello"}"#);
```

## Testing

`tiny-json-rs` comes with a suite of tests to ensure functionality:

```rust
#[test]
fn test_deserialize() {
    // ...
}

#[test]
fn test_deserialize_vec() {
    // ...
}

#[test]
fn test_encode_json() {
    // ...
}
```

## License

`tiny-json-rs` is licensed under MIT.

## Issues?
This is an experimental project, not production ready yet. If you find any issues, please report them [here](https://github.com/EdsonHTJ/tiny-json-rs/issues)
