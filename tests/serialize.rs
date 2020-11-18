use serde::{Deserialize, Serialize};
use serde_bytes_repr::ByteFmtSerializer;

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
    let ser = ByteFmtSerializer::base64(&mut ser, base64_config);
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
    let ser = ByteFmtSerializer::base64(&mut ser, base64_config);
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
    let ser = ByteFmtSerializer::hex(&mut ser);
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
    let ser = ByteFmtSerializer::hex(&mut ser);
    demo.serialize(ser).unwrap();

    let serialized = String::from_utf8(out).unwrap();
    assert_eq!(r#""74657374696e67""#, serialized.as_str());
}

#[test]
fn serialize_option_base64() {
    #[derive(Serialize, Deserialize)]
    struct Demo {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
    }
    let bytes = b"testing".to_vec();
    let demo = Some(Demo { bytes });

    let mut out = vec![];
    let mut ser = serde_json::Serializer::new(&mut out);
    let base64_config = base64::Config::new(base64::CharacterSet::UrlSafe, true);
    let ser = ByteFmtSerializer::base64(&mut ser, base64_config);
    demo.serialize(ser).unwrap();

    let serialized = String::from_utf8(out).unwrap();
    assert_eq!(r#"{"bytes":"dGVzdGluZw=="}"#, serialized.as_str());
}

#[test]
fn serialize_option_hex() {
    #[derive(Serialize, Deserialize)]
    struct Demo {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
    }
    let bytes = b"testing".to_vec();
    let demo = Some(Demo { bytes });

    let mut out = vec![];
    let mut ser = serde_json::Serializer::new(&mut out);
    let ser = ByteFmtSerializer::hex(&mut ser);
    demo.serialize(ser).unwrap();

    let serialized = String::from_utf8(out).unwrap();
    assert_eq!(r#"{"bytes":"74657374696e67"}"#, serialized.as_str());
}
