use serde::{Deserialize, Serialize};
use serde_bytes_repr::ByteFmtDeserializer;

#[test]
fn deserialize_struct_base64() {
    #[derive(Serialize, Deserialize)]
    struct Demo {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
    }

    let json = br#"{"bytes":"dGVzdGluZw=="}"#;
    let mut json_de = serde_json::Deserializer::from_slice(json);
    let base64_config = base64::engine::GeneralPurposeConfig::new();
    let bytefmt_json_de =
        ByteFmtDeserializer::new_base64(&mut json_de, base64::alphabet::URL_SAFE, base64_config);
    let demo: Demo = Demo::deserialize(bytefmt_json_de).unwrap();

    let deserialized = String::from_utf8(demo.bytes).unwrap();
    assert_eq!("testing", deserialized.as_str());
}

#[test]
fn deserialize_seq_base64() {
    let json = br#""dGVzdGluZw==""#;
    let mut json_de = serde_json::Deserializer::from_slice(json);
    let base64_config = base64::engine::GeneralPurposeConfig::new();
    let bytefmt_json_de =
        ByteFmtDeserializer::new_base64(&mut json_de, base64::alphabet::URL_SAFE, base64_config);
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

#[test]
fn deserialize_option_base64() {
    #[derive(Serialize, Deserialize)]
    struct Demo {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
    }

    #[derive(Serialize, Deserialize)]
    struct OptionWrapper {
        demo: Option<Demo>,
    }

    let json = br#"{"demo":{"bytes":"dGVzdGluZw=="}}"#;
    let mut json_de = serde_json::Deserializer::from_slice(json);
    let base64_config = base64::engine::GeneralPurposeConfig::new();
    let bytefmt_json_de =
        ByteFmtDeserializer::new_base64(&mut json_de, base64::alphabet::URL_SAFE, base64_config);
    let option_wrapper: OptionWrapper = OptionWrapper::deserialize(bytefmt_json_de).unwrap();

    let deserialized = String::from_utf8(option_wrapper.demo.unwrap().bytes).unwrap();
    assert_eq!("testing", deserialized.as_str());
}

#[test]
fn deserialize_option_hex() {
    #[derive(Serialize, Deserialize)]
    struct Demo {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
    }

    #[derive(Serialize, Deserialize)]
    struct OptionWrapper {
        demo: Option<Demo>,
    }

    let json = br#"{"demo":{"bytes":"74657374696e67"}}"#;
    let mut json_de = serde_json::Deserializer::from_slice(json);
    let bytefmt_json_de = ByteFmtDeserializer::new_hex(&mut json_de);
    let option_wrapper: OptionWrapper = OptionWrapper::deserialize(bytefmt_json_de).unwrap();

    let deserialized = String::from_utf8(option_wrapper.demo.unwrap().bytes).unwrap();
    assert_eq!("testing", deserialized.as_str());
}

#[test]
fn deserialize_invalid_struct_base64() {
    #[derive(Serialize, Deserialize, Debug)]
    struct Demo {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
    }

    let json = br#"{"bytes":"12345"}"#;
    let mut json_de = serde_json::Deserializer::from_slice(json);
    let base64_config = base64::engine::GeneralPurposeConfig::new();
    let bytefmt_json_de =
        ByteFmtDeserializer::new_base64(&mut json_de, base64::alphabet::URL_SAFE, base64_config);
    let demo = Demo::deserialize(bytefmt_json_de);

    let msg = format!("{}", demo.unwrap_err());
    assert_eq!(
        "invalid length 5, expected valid base64 length at line 1 column 16",
        msg
    );

    let json = br#"{"bytes":"12345%"}"#;
    let mut json_de = serde_json::Deserializer::from_slice(json);
    let base64_config = base64::engine::GeneralPurposeConfig::new();
    let bytefmt_json_de =
        ByteFmtDeserializer::new_base64(&mut json_de, base64::alphabet::URL_SAFE, base64_config);
    let demo = Demo::deserialize(bytefmt_json_de);

    let msg = format!("{}", demo.unwrap_err());
    assert_eq!("invalid value: character `%`, expected valid base64 character at index 5 at line 1 column 17", msg);

    let json = br#"{"bytes":"123456"}"#;
    let mut json_de = serde_json::Deserializer::from_slice(json);
    let base64_config = base64::engine::GeneralPurposeConfig::new()
        .with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent);
    let bytefmt_json_de =
        ByteFmtDeserializer::new_base64(&mut json_de, base64::alphabet::URL_SAFE, base64_config);
    let demo = Demo::deserialize(bytefmt_json_de);

    let msg = format!("{}", demo.unwrap_err());
    assert_eq!("invalid value: character `6`, expected valid character ending base64 string at line 1 column 17", msg);
}

#[test]
fn deserialize_invalid_struct_hex() {
    #[derive(Serialize, Deserialize, Debug)]
    struct Demo {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
    }

    let json = br#"{"bytes":"74657374696e6"}"#;
    let mut json_de = serde_json::Deserializer::from_slice(json);
    let bytefmt_json_de = ByteFmtDeserializer::new_hex(&mut json_de);
    let demo = Demo::deserialize(bytefmt_json_de);

    let msg = format!("{}", demo.unwrap_err());
    assert_eq!(
        "invalid length 13, expected even length at line 1 column 24",
        msg
    );

    let json = br#"{"bytes":"746g7374696e67"}"#;
    let mut json_de = serde_json::Deserializer::from_slice(json);
    let bytefmt_json_de = ByteFmtDeserializer::new_hex(&mut json_de);
    let demo = Demo::deserialize(bytefmt_json_de);

    let msg = format!("{}", demo.unwrap_err());
    assert_eq!(
        "invalid value: character `g`, expected valid hex character at index 3 at line 1 column 25",
        msg
    );
}
