# Serde adapter for controlling the representation of bytes

[![Build Status](https://travis-ci.com/cfsamson/rfi-serde-byte-repr.svg?branch=master)](https://travis-ci.com/cfsamson/rfi-serde-byte-repr)

Human readable formats tend not to include a universally agreed way to represent arbitrary binary
data, which means those serde libraries can end up using a representation for serde's "bytes" type
which isn't ideal for all uses.

Three examples:

```rust
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_bytes = "0.11"
// serde_json = "1.0"
// serde_yaml = "0.8"
// toml = "0.5"

use serde::Serialize;

#[derive(Serialize)]
struct Demo {
    #[serde(with = "serde_bytes")]
    bytes: Vec<u8>,
}

fn main() {
    let bytes = b"testing".to_vec();
    let s = Demo { bytes };

    println!("JSON: {}", serde_json::to_string(&s).unwrap());
    println!("YAML: {}", serde_yaml::to_string(&s).unwrap());
    println!("TOML: {}", toml::to_string(&s).unwrap());
}
```

```
JSON: {"bytes":[116,101,115,116,105,110,103]}
YAML: ---
bytes:
  - 116
  - 101
  - 115
  - 116
  - 105
  - 110
  - 103
TOML: bytes = [116, 101, 115, 116, 105, 110, 103]
```

This adapter let's you control how the bytes are represented by wrapping a serializer like this:

```rust
#[derive(Serialize, Deserialize)]
struct Demo {
    #[serde(with = "serde_bytes")]
    bytes: Vec<u8>,
}

fn main() {
    let bytes = b"testing".to_vec();
    let demo = Demo { bytes };

    let mut out = vec![];
    let mut ser = serde_json::Serializer::new(&mut out);
    let base64_config = base64::Config::new(base64::CharacterSet::UrlSafe, true);
    let ser = BytesRepr::base64(&mut ser, base64_config);
    demo.serialize(ser).unwrap();

    let serialized = String::from_utf8(out).unwrap();
    println!("JSON(base64): {}", serialized);
}
```

Outputs:

```
JSON(base64): {"bytes":"dGVzdGluZw=="}
```
