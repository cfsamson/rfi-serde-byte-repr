use serde::{Deserialize, Serialize};
use serde_byte_repr::{ByteFmtDeserializer, ByteFmtSerializer};

#[test]
fn deserialize_struct_base64() {
    #[derive(Serialize, Deserialize)]
    struct Demo {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
    }

    let json = br#"{"bytes":"dGVzdGluZw=="}"#;
    let mut json_de = serde_json::Deserializer::from_slice(json);
    let base64_config = base64::Config::new(base64::CharacterSet::UrlSafe, true);
    let bytefmt_json_de = ByteFmtDeserializer::new_base64(&mut json_de, base64_config);
    let demo: Demo = Demo::deserialize(bytefmt_json_de).unwrap();

    let deserialized = String::from_utf8(demo.bytes).unwrap();
    assert_eq!("testing", deserialized.as_str());
}

#[test]
fn deserialize_seq_base64() {
    let json = br#""dGVzdGluZw==""#;
    let mut json_de = serde_json::Deserializer::from_slice(json);
    let base64_config = base64::Config::new(base64::CharacterSet::UrlSafe, true);
    let bytefmt_json_de = ByteFmtDeserializer::new_base64(&mut json_de, base64_config);
    let bytes: serde_bytes::ByteBuf = Deserialize::deserialize(bytefmt_json_de).unwrap();

    let deserialized = String::from_utf8(bytes.to_vec()).unwrap();
    assert_eq!("testing", deserialized.as_str());
}

#[test]
fn deserialize_struct_hex() {
    #[derive(Serialize, Deserialize)]
    struct Demo {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
    }

    let json = br#"{"bytes":"74657374696e67"}"#;
    let mut json_de = serde_json::Deserializer::from_slice(json);
    let bytefmt_json_de = ByteFmtDeserializer::new_hex(&mut json_de);
    let demo: Demo = Demo::deserialize(bytefmt_json_de).unwrap();

    let deserialized = String::from_utf8(demo.bytes).unwrap();
    assert_eq!("testing", deserialized.as_str());
}

#[test]
fn serialize_seq_hex() {
    let json = br#""74657374696e67""#;
    let mut json_de = serde_json::Deserializer::from_slice(json);
    let bytefmt_json_de = ByteFmtDeserializer::new_hex(&mut json_de);
    let bytes: serde_bytes::ByteBuf = Deserialize::deserialize(bytefmt_json_de).unwrap();

    let deserialized = String::from_utf8(bytes.to_vec()).unwrap();
    assert_eq!("testing", deserialized.as_str());
}
