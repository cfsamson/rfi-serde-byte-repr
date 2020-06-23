
mod serializer;

#[derive(Clone)]
enum EncodeKind {
    Base64(base64::Config),
    Hex,
}

pub struct BytesRepr<S> {
    inner: S,
    encode_kind: EncodeKind,
}
impl<S> BytesRepr<S> {
    pub fn base64(ser: S, cfg: base64::Config) -> Self {
        Self {
            inner: ser,
            encode_kind: EncodeKind::Base64(cfg),
        }
    }

    pub fn hex(ser: S) -> Self {
        Self { inner: ser, encode_kind: EncodeKind::Hex }
    }

    fn encode(&self, v: &[u8]) -> String {
        match self.encode_kind {
            EncodeKind::Base64(cfg) => base64::encode_config(&v, cfg),
            EncodeKind::Hex => hex::encode(&v),
        }
    }
}