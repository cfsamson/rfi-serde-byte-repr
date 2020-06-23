use core::fmt;
use fmt::Display;
use serde::{
    ser::{Error, self},
    serde_if_integer128, Serialize, Serializer,
};

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

impl<S: Serializer> Serializer for BytesRepr<S> {
    type Ok = S::Ok;
    type Error = S::Error;

    type SerializeSeq = SerializeSeq<S::SerializeSeq>;
    type SerializeTuple = SerializeTuple<S::SerializeTuple>;
    type SerializeTupleStruct = SerializeTupleStruct<S::SerializeTupleStruct>;
    type SerializeTupleVariant = SerializeTupleVariant<S::SerializeTupleVariant>;
    type SerializeMap = SerializeMap<S::SerializeMap>;
    type SerializeStruct = SerializeStruct<S::SerializeStruct>;
    type SerializeStructVariant = SerializeStructVariant<S::SerializeStructVariant>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        S::serialize_bool(self.inner, v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        S::serialize_i8(self.inner, v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        S::serialize_i16(self.inner, v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        S::serialize_i32(self.inner, v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        S::serialize_i64(self.inner, v)
    }

    serde_if_integer128! {
        fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
            let _ = v;
            Err(Error::custom("i128 is not supported"))
        }
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        S::serialize_u8(self.inner, v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        S::serialize_u16(self.inner, v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        S::serialize_u32(self.inner, v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        S::serialize_u64(self.inner, v)
    }

    serde_if_integer128! {
        fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
            let _ = v;
            Err(Error::custom("u128 is not supported"))
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        S::serialize_f32(self.inner, v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        S::serialize_f64(self.inner, v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        S::serialize_char(self.inner, v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        S::serialize_str(self.inner, v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let encoded = self.encode(v);
        S::serialize_str(self.inner, &encoded)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        S::serialize_none(self.inner)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        S::serialize_some(self.inner, value)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        S::serialize_unit(self.inner)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        S::serialize_unit_struct(self.inner, name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        S::serialize_unit_variant(self.inner, name, variant_index, variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        S::serialize_newtype_struct(self.inner, name, value)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        S::serialize_newtype_variant(self.inner, name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let kind = self.encode_kind;
        S::serialize_seq(self.inner, len).map(|ser| SerializeSeq::new(ser, kind))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let kind = self.encode_kind;
        S::serialize_tuple(self.inner, len).map(|ser| SerializeTuple::new(ser, kind))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let kind = self.encode_kind;
        S::serialize_tuple_struct(self.inner, name, len).map(|ser| SerializeTupleStruct::new(ser, kind))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let kind = self.encode_kind;
        S::serialize_tuple_variant(self.inner, name, variant_index, variant, len).map(|ser| SerializeTupleVariant::new(ser, kind))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let kind = self.encode_kind;
        S::serialize_map(self.inner, len).map(|ser| SerializeMap::new(ser, kind))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let encode_kind = self.encode_kind;
        S::serialize_struct(self.inner, name, len).map(|ser| SerializeStruct::new(ser, encode_kind))
    }

    /// ```
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let kind = self.encode_kind;
        S::serialize_struct_variant(self.inner, name, variant_index, variant, len).map(|ser| SerializeStructVariant::new(ser, kind))
    }

    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        let kind = self.encode_kind;
        let iter = iter.into_iter().map(|item| BytesSerializeSized::new(item, kind.clone()));
        self.inner.collect_seq(iter)
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        let kind = self.encode_kind;
        let iter = iter.into_iter()
        .map(|(k, v)| {(
            BytesSerializeSized::new(k, kind.clone()),
            BytesSerializeSized::new(v, kind.clone()),
        )});
        self.inner.collect_map(iter)
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display,
    {
        self.inner.serialize_str(&value.to_string())
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display,
    {
        S::collect_str(self.inner, value)
    }

    fn is_human_readable(&self) -> bool {
        self.inner.is_human_readable()
    }
}

pub struct BytesSerialize<'a, T: ?Sized> {
    value: &'a T,
    kind: EncodeKind,
}

impl<'a, T: ?Sized> BytesSerialize<'a, T> {
    fn new(value: &'a T, kind: EncodeKind) -> Self {
        BytesSerialize { value, kind }
    }
}

impl<'a, T: ?Sized> ser::Serialize for BytesSerialize<'a, T> 
    where T: ser::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        ser::Serialize::serialize(self.value, BytesRepr {
            inner: serializer,
            encode_kind: self.kind.clone(),
        })
    }
}


struct BytesSerializeSized<T> {
    value: T,
    kind: EncodeKind,
}

impl<T> BytesSerializeSized<T> {
    fn new(value: T, kind: EncodeKind) -> Self {
        BytesSerializeSized { value, kind }
    }
}

impl<T> ser::Serialize for BytesSerializeSized<T>
where
    T: ser::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
            ser::Serialize::serialize(&self.value, BytesRepr {
                inner: serializer,
                encode_kind: self.kind.clone(),
            })
    }
}

pub struct SerializeSeq<S> {
    ser: S,
    kind: EncodeKind,
}

impl<S> SerializeSeq<S> {
    fn new(ser: S, kind: EncodeKind) -> Self {
        SerializeSeq { ser, kind }
    }
}

impl<S> ser::SerializeSeq for SerializeSeq<S>
where
    S: ser::SerializeSeq,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser
            .serialize_element(&BytesSerialize::new(value, self.kind.clone()))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }
}

pub struct SerializeTuple<S> {
    ser: S,
    kind: EncodeKind,
}

impl<S> SerializeTuple<S> {
    fn new(serialize_tuple: S, kind: EncodeKind) -> Self {
        SerializeTuple {
            ser: serialize_tuple,
            kind,
        }
    }
}

impl<S> ser::SerializeTuple for SerializeTuple<S>
where
    S: ser::SerializeTuple,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser
            .serialize_element(&BytesSerialize::new(value, self.kind.clone()))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }
}

pub struct SerializeTupleStruct<S> {
    ser: S,
    kind: EncodeKind,
}

impl<S> SerializeTupleStruct<S> {
    fn new(serialize_tuple_struct: S, kind: EncodeKind) -> Self {
        SerializeTupleStruct {
            ser: serialize_tuple_struct,
            kind,
        }
    }
}

impl<S> ser::SerializeTupleStruct for SerializeTupleStruct<S>
where
    S: ser::SerializeTupleStruct,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_field(&BytesSerialize::new(value, self.kind.clone()))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }
}

pub struct SerializeTupleVariant<S> {
    ser: S,
    kind: EncodeKind,
}

impl<S> SerializeTupleVariant<S> {
    fn new(serialize_tuple_variant: S, kind: EncodeKind) -> Self {
        SerializeTupleVariant {
            ser: serialize_tuple_variant,
            kind,
        }
    }
}

impl<S> ser::SerializeTupleVariant for SerializeTupleVariant<S>
where
    S: ser::SerializeTupleVariant,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_field(&BytesSerialize::new(value, self.kind.clone()))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }
}

pub struct SerializeMap<S> {
    ser: S,
    kind: EncodeKind,
}

impl<S> SerializeMap<S> {
    fn new(serialize_map: S, kind: EncodeKind) -> Self {
        SerializeMap {
            ser: serialize_map,
            kind,
        }
    }
}

impl<S> ser::SerializeMap for SerializeMap<S>
where
    S: ser::SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_key(&BytesSerialize::new(key, self.kind.clone()))
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_value(&BytesSerialize::new(value, self.kind.clone()))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + ser::Serialize,
        V: ?Sized + ser::Serialize,
    {
        self.ser.serialize_entry(
            &BytesSerialize::new(key, self.kind.clone()),
            &BytesSerialize::new(value, self.kind.clone()),
        )
    }
}

pub struct SerializeStruct<S> {
    ser: S,
    encode_kind: EncodeKind,
}

impl<S> SerializeStruct<S> {
    fn new(ser: S, encode_kind: EncodeKind) -> Self {
        SerializeStruct { ser, encode_kind }
    }
}

impl<S: ser::SerializeStruct> ser::SerializeStruct for SerializeStruct<S> {
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser
            .serialize_field(key, &BytesSerialize::new(value, self.encode_kind.clone()))
    }

      fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.ser.skip_field(key)
    }
}

pub struct SerializeStructVariant<S> {
    ser: S,
    kind: EncodeKind,
}

impl<S> SerializeStructVariant<S> {
    fn new(serialize_struct_variant: S, kind: EncodeKind) -> Self {
        SerializeStructVariant {
            ser: serialize_struct_variant,
            kind,
        }
    }
}

impl<S> ser::SerializeStructVariant for SerializeStructVariant<S>
where
    S: ser::SerializeStructVariant,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser
            .serialize_field(key, &BytesSerialize::new(value, self.kind.clone()))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.ser.skip_field(key)
    }
}
