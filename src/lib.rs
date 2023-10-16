//! # Serde adapter for (de)serializing bytes to and from specific formats
//!
//! Currently these representations is supported:
//!
//! - Base64
//! - Hexidecimal
//!
//! Human readable formats tend not to include a universally agreed way to represent arbitrary binary
//! data, which means those serde libraries can end up using a representation for serde's "bytes" type
//! which isn't ideal for all uses. This library gives you the option to choose a different
//! representation than the default for libraries like `serde_json`, `toml` and `serde_yaml`.
//!
//! ## How to make sure that your datatype is interpreted as bytes?
//!
//! Without specialization, Rust forces Serde to treat &[u8] just like any other
//! slice and Vec<u8> just like any other vector. To enable specialized handling
//! of `&[u8]` can use the [serde_bytes](https://docs.rs/serde_bytes/0.11.5/serde_bytes/index.html) crate.
//!
//! You'll se this in use in the examples below:
//!
//! ## Serialization
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serde_bytes_repr::ByteFmtSerializer;
//! # fn main() {
//!     #[derive(Serialize, Deserialize)]
//!     struct Demo {
//!         #[serde(with = "serde_bytes")]
//!         bytes: Vec<u8>,
//!     }
//!     let bytes = b"testing".to_vec();
//!     let demo = Demo { bytes };
//!
//!     let mut out = vec![];
//!     let mut ser = serde_json::Serializer::new(&mut out);
//!     let base64_config = base64::engine::GeneralPurposeConfig::new();
//!     let ser = ByteFmtSerializer::base64(&mut ser,  base64::alphabet::URL_SAFE, base64_config);
//!     demo.serialize(ser).unwrap();
//!
//!     let serialized = String::from_utf8(out).unwrap();
//!     assert_eq!(r#"{"bytes":"dGVzdGluZw=="}"#, serialized.as_str());
//! # }
//! ```
//!
//! ## Deserialization
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serde_bytes_repr::{ByteFmtDeserializer, ByteFmtSerializer};
//! # fn main() {
//!     #[derive(Serialize, Deserialize)]
//!     struct Demo {
//!         #[serde(with = "serde_bytes")]
//!         bytes: Vec<u8>,
//!     }
//!
//!     let json = br#"{"bytes":"dGVzdGluZw=="}"#;
//!     let mut json_de = serde_json::Deserializer::from_slice(json);
//!     let base64_config = base64::engine::GeneralPurposeConfig::new();
//!     let bytefmt_json_de = ByteFmtDeserializer::new_base64(&mut json_de, base64::alphabet::URL_SAFE, base64_config);
//!     let demo: Demo = Demo::deserialize(bytefmt_json_de).unwrap();
//!
//!     let deserialized = String::from_utf8(demo.bytes).unwrap();
//!     assert_eq!("testing", deserialized.as_str());
//! # }
//! ```

use base64::{alphabet::Alphabet, engine::GeneralPurposeConfig, Engine};

mod deserializer;
mod serializer;

#[derive(Clone)]
enum ByteFormat {
    Base64(Alphabet, GeneralPurposeConfig),
    Hex,
}

/// Serializer-adapter which encodes bytes to using the specified encoding. The format is
/// serialized to the data formats string representation.
pub struct ByteFmtSerializer<S> {
    inner: S,
    encode_kind: ByteFormat,
}
impl<S> ByteFmtSerializer<S> {
    /// Crates an adapter which serializes to and from a Base64 representation.
    /// Provide a configuration from the `base64` crate specifying the specifics
    /// on how you want the bytes encoded.
    pub fn base64(ser: S, alphabet: Alphabet, config: GeneralPurposeConfig) -> Self {
        Self {
            inner: ser,
            encode_kind: ByteFormat::Base64(alphabet, config),
        }
    }

    /// Creates an adapter which serializes to a HEX representation.
    pub fn hex(ser: S) -> Self {
        Self {
            inner: ser,
            encode_kind: ByteFormat::Hex,
        }
    }

    fn encode(&self, v: &[u8]) -> String {
        match self.encode_kind {
            ByteFormat::Base64(ref alphabet, config) => {
                base64::engine::GeneralPurpose::new(alphabet, config).encode(v)
            }
            ByteFormat::Hex => hex::encode(v),
        }
    }
}

/// Deserializer-adapter which decodes bytes from a specified format.
pub struct ByteFmtDeserializer<D> {
    pub inner: D,
    fmt: ByteFormat,
}

impl<D> ByteFmtDeserializer<D> {
    /// Crates an adapter which deserializes from a Base64 representation. Provide a
    /// configuration from the `base64` crate specifying the specifics on how you want the bytes
    /// encoded.
    ///
    pub fn new_base64(deserializer: D, alphabet: Alphabet, config: GeneralPurposeConfig) -> Self {
        ByteFmtDeserializer {
            inner: deserializer,
            fmt: ByteFormat::Base64(alphabet, config),
        }
    }

    /// Creates an adapter which deserializes from a HEX representation.
    pub fn new_hex(deserializer: D) -> Self {
        ByteFmtDeserializer {
            inner: deserializer,
            fmt: ByteFormat::Hex,
        }
    }
}
