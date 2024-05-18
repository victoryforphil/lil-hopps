use std::collections::BTreeMap;

use serde::{de::value::Error, ser, Serialize};

use crate::Primatives;

pub struct PrimativeSerializer {
    // This string starts empty and JSON is appended as values are serialized.
    output: BTreeMap<String, Primatives>,
}


pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: Serialize,
{
    let mut serializer = PrimativeSerializer {
        output: BTreeMap::new(),
    };

    value.serialize(&mut serializer)?;

    let mut output = String::from("{");
    for (key, value) in serializer.output.iter() {
        output.push_str(&format!("\"{}\":{:?},", key, value));
    }
    output.push_str("}");

    Ok(output)
}

impl<'a> ser::Serializer for &'a mut PrimativeSerializer {
     // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = Primatives;

   // The error type when some error occurs during serialization.
    type Error = Error;

  // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::Boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::Number(v as f64))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::Number(v as f64))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::Number(v as f64))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::Number(v as f64))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::Number(v as f64))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::Number(v as f64))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::Number(v as f64))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::Number(v as f64))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::Number(v as f64))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::Number(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::String(v.to_string()))   
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        // Serialize to Number Array
        let v: Vec<f64> = v.iter().map(|&x| x as f64).collect();
        Ok(Primatives::NumberArray(v))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::String("null".to_string()))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize {
            value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Primatives::String("null".to_string()))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
       self.serialize_unit()    
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}