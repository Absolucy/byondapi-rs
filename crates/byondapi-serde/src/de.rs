use crate::error::DeserializeError;
use byondapi::{ByondTypeCheck, ByondValue, ByondValueType, FromByond};
use serde::de::{
	self, DeserializeSeed, Deserializer, EnumAccess, MapAccess, SeqAccess, VariantAccess,
};
use std::collections::{HashMap, VecDeque};

pub(crate) struct ByondDeserializer {
	pub(crate) value: ByondValue,
}

impl<'de, 'a> Deserializer<'de> for &'a ByondDeserializer {
	type Error = DeserializeError;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let val = self.value.get_type();
		match val {
			ByondValueType::NULL => visitor.visit_unit(),
			ByondValueType::STRING => self.deserialize_string(visitor),
			ByondValueType::NUMBER => self.deserialize_f32(visitor),
			ByondValueType::LIST => self.deserialize_seq(visitor),
			_ => Err(DeserializeError::Unexpected("any type".into(), val.name())),
		}
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		bool::from_byond(&self.value)
			.map_err(DeserializeError::from)
			.and_then(|value| visitor.visit_bool(value))
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		i8::from_byond(&self.value)
			.map_err(DeserializeError::from)
			.and_then(|value| visitor.visit_i8(value))
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		i16::from_byond(&self.value)
			.map_err(DeserializeError::from)
			.and_then(|value| visitor.visit_i16(value))
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		i32::from_byond(&self.value)
			.map_err(DeserializeError::from)
			.and_then(|value| visitor.visit_i32(value))
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		i64::from_byond(&self.value)
			.map_err(DeserializeError::from)
			.and_then(|value| visitor.visit_i64(value))
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		u8::from_byond(&self.value)
			.map_err(DeserializeError::from)
			.and_then(|value| visitor.visit_u8(value))
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		u16::from_byond(&self.value)
			.map_err(DeserializeError::from)
			.and_then(|value| visitor.visit_u16(value))
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		u32::from_byond(&self.value)
			.map_err(DeserializeError::from)
			.and_then(|value| visitor.visit_u32(value))
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		u64::from_byond(&self.value)
			.map_err(DeserializeError::from)
			.and_then(|value| visitor.visit_u64(value))
	}

	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		f32::from_byond(&self.value)
			.map_err(DeserializeError::from)
			.and_then(|value| visitor.visit_f32(value))
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		f64::from_byond(&self.value)
			.map_err(DeserializeError::from)
			.and_then(|value| visitor.visit_f64(value))
	}

	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		String::from_byond(&self.value)
			.map_err(DeserializeError::from)
			.and_then(|value| value.chars().next().ok_or(DeserializeError::EndOfArray))
			.and_then(|value| visitor.visit_char(value))
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		if !self.value.is_str() {
			panic!("oh noes this wasn't a string");
		}
		let ptr = unsafe { byondapi::byond().ByondValue_GetStr(&self.value.0) };
		let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
		let string = cstr.to_str().expect("oh noes! invalid string");
		visitor.visit_string(string.to_owned())
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		if !self.value.is_str() {
			panic!("oh noes this wasn't a string");
		}
		let ptr = unsafe { byondapi::byond().ByondValue_GetStr(&self.value.0) };
		let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
		let string = cstr.to_str().expect("oh noes! invalid string");
		visitor.visit_string(string.to_owned())
	}

	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		Vec::<u8>::from_byond(&self.value)
			.map_err(DeserializeError::from)
			.and_then(|value| visitor.visit_byte_buf(value))
	}

	#[inline]
	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_bytes(visitor)
	}

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		if self.value.is_null() {
			visitor.visit_none()
		} else {
			visitor.visit_some(self)
		}
	}

	#[inline]
	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_unit()
	}

	#[inline]
	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_unit()
	}

	#[inline]
	fn deserialize_unit_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_unit()
	}

	#[inline]
	fn deserialize_newtype_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let iter = self.value.iter()?;
		let vec: VecDeque<_> = iter.map(|(key, _)| key).collect();
		visitor.visit_seq(ByondSeqAccess { iter: vec })
	}

	#[inline]
	fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	#[inline]
	fn deserialize_tuple_struct<V>(
		self,
		_name: &'static str,
		len: usize,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_tuple(len, visitor)
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let iter = self.value.iter()?;
		let map: HashMap<_, _> = iter.collect();
		visitor.visit_map(ByondMapAccess {
			iter: map.into_iter(),
			current: None,
		})
	}

	#[inline]
	fn deserialize_struct<V>(
		self,
		_name: &'static str,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_map(visitor)
	}

	#[inline]
	fn deserialize_enum<V>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_enum(ByondEnumAccess { deserializer: self })
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		std::thread::yield_now(); // this is needed in order to not crash. just... don't think about it too hard.
		self.deserialize_str(visitor)
	}
}

// Additional structs for SeqAccess and MapAccess
struct ByondSeqAccess {
	iter: VecDeque<ByondValue>,
}

impl<'de> SeqAccess<'de> for ByondSeqAccess {
	type Error = DeserializeError;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
	where
		T: DeserializeSeed<'de>,
	{
		if let Some(value) = self.iter.pop_front() {
			eprintln!(
				"ByondSeqAccess::next_element_seed: {}",
				value.get_type().name()
			);
			let deserializer = ByondDeserializer { value };
			seed.deserialize(&deserializer).map(Some)
		} else {
			Ok(None)
		}
	}
}

struct ByondMapAccess {
	iter: std::collections::hash_map::IntoIter<ByondValue, Option<ByondValue>>,
	current: Option<(ByondValue, Option<ByondValue>)>,
}

impl<'de> MapAccess<'de> for ByondMapAccess {
	type Error = DeserializeError;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: DeserializeSeed<'de>,
	{
		self.current = self.iter.next();
		if let Some((key, _)) = &self.current {
			let deserializer = ByondDeserializer { value: key.clone() };
			seed.deserialize(&deserializer).map(Some)
		} else {
			Ok(None)
		}
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: DeserializeSeed<'de>,
	{
		let value = self
			.current
			.as_ref()
			.map(|(_, v)| v)
			.ok_or(DeserializeError::EndOfArray)?;
		let deserializer = ByondDeserializer {
			value: value.clone().unwrap_or_default(),
		};
		seed.deserialize(&deserializer)
	}
}

struct ByondEnumAccess<'a> {
	deserializer: &'a ByondDeserializer,
}

impl<'de, 'a> EnumAccess<'de> for ByondEnumAccess<'a> {
	type Error = DeserializeError;
	type Variant = Self;

	#[inline]
	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
	where
		V: DeserializeSeed<'de>,
	{
		let variant: <V as DeserializeSeed>::Value = seed.deserialize(self.deserializer)?;
		Ok((variant, self))
	}
}

impl<'de, 'a> VariantAccess<'de> for ByondEnumAccess<'a> {
	type Error = DeserializeError;

	#[inline]
	fn unit_variant(self) -> Result<(), Self::Error> {
		Ok(())
	}

	#[inline]
	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
	where
		T: DeserializeSeed<'de>,
	{
		seed.deserialize(self.deserializer)
	}

	#[inline]
	fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		de::Deserializer::deserialize_seq(self.deserializer, visitor)
	}

	fn struct_variant<V>(
		self,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		de::Deserializer::deserialize_map(self.deserializer, visitor)
	}
}
