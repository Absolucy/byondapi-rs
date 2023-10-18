use super::ByondSerializer;
use crate::error::SerializeError;
use byondapi::{byondval, ByondResult, ByondValue, ToByond};
use serde::ser::{Serialize, SerializeMap, SerializeStruct, SerializeStructVariant};

pub(crate) struct ByondMapSerializer<'a> {
	pub serializer: &'a mut ByondSerializer,
	pub map: ByondValue,
	pub variant: Option<&'static str>,
	pub key: ByondValue,
}

impl<'a> ByondMapSerializer<'a> {
	pub fn new(
		serializer: &'a mut ByondSerializer,
		variant: Option<&'static str>,
	) -> ByondResult<Self> {
		Ok(Self {
			serializer,
			map: ByondValue::new_list()?,
			variant,
			key: ByondValue::null(),
		})
	}
}

impl<'a> SerializeMap for ByondMapSerializer<'a> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		self.key = key.serialize(&mut *self.serializer)?;
		Ok(())
	}

	fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		let key = std::mem::take(&mut self.key);
		let value = value.serialize(&mut *self.serializer)?;
		self.map
			.write_list_index_internal(&key, &value)
			.map_err(SerializeError::from)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.map)
	}
}

impl<'a> SerializeStruct for ByondMapSerializer<'a> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_field<T: ?Sized>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		let key = key.to_byond()?;
		let value = value.serialize(&mut *self.serializer)?;
		self.map
			.write_list_index_internal(&key, &value)
			.map_err(SerializeError::from)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.map)
	}
}

impl<'a> SerializeStructVariant for ByondMapSerializer<'a> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_field<T: ?Sized>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		let key = key.to_byond()?;
		let value = value.serialize(&mut *self.serializer)?;
		self.map
			.write_list_index_internal(&key, &value)
			.map_err(SerializeError::from)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		let mut list = ByondValue::new_list()?;
		if let Some(variant) = self.variant.map(|variant| byondval!(variant)) {
			list.write_list_index_internal(&variant, &self.map)?;
		}
		Ok(list)
	}
}
