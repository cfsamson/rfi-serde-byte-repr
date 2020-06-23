use serde_byte_repr::{BytesRepr};
use serde::{Serialize, Deserialize};

#[test]
fn serialize_struct_base64() {
    #[derive(Serialize, Deserialize)]
    struct Demo {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
    }
    let bytes = b"testing".to_vec();
    let demo = Demo { bytes };

    let mut out = vec![];
    let mut ser = serde_json::Serializer::new(&mut out);
    let base64_config = base64::Config::new(base64::CharacterSet::UrlSafe, true);
    let ser = BytesRepr::base64(&mut ser, base64_config);
    demo.serialize(ser).unwrap();

    let serialized = String::from_utf8(out).unwrap();
    assert_eq!(r#"{"bytes":"dGVzdGluZw=="}"#, serialized.as_str());
}

#[test]
fn serialize_seq_base64() {
    let bytes = b"testing".to_vec();
    let demo = serde_bytes::ByteBuf::from(bytes);

    let mut out = vec![];
    let mut ser = serde_json::Serializer::new(&mut out);
    let base64_config = base64::Config::new(base64::CharacterSet::UrlSafe, true);
    let ser = BytesRepr::base64(&mut ser, base64_config);
    demo.serialize(ser).unwrap();

    let serialized = String::from_utf8(out).unwrap();
    assert_eq!(r#""dGVzdGluZw==""#, serialized.as_str());
}

#[test]
fn serialize_struct_hex() {
    #[derive(Serialize, Deserialize)]
    struct Demo {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
    }
    let bytes = b"testing".to_vec();
    let demo = Demo { bytes };

    let mut out = vec![];
    let mut ser = serde_json::Serializer::new(&mut out);
    let ser = BytesRepr::hex(&mut ser);
    demo.serialize(ser).unwrap();

    let serialized = String::from_utf8(out).unwrap();
    assert_eq!(r#"{"bytes":"74657374696e67"}"#, serialized.as_str());
}

#[test]
fn serialize_seq_hex() {
    let bytes = b"testing".to_vec();
    let demo = serde_bytes::ByteBuf::from(bytes);

    let mut out = vec![];
    let mut ser = serde_json::Serializer::new(&mut out);
    let ser = BytesRepr::hex(&mut ser);
    demo.serialize(ser).unwrap();

    let serialized = String::from_utf8(out).unwrap();
    assert_eq!(r#""74657374696e67""#, serialized.as_str());
}