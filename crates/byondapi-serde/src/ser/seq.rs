use super::ByondSerializer;
use crate::error::SerializeError;
use byondapi::{prelude::ByondValueList, ByondValue, ToByond};
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, SerializeTupleStruct};

pub(crate) struct ByondSeqSerializer<'a> {
	serializer: &'a mut ByondSerializer,
	list: ByondValueList,
}

impl<'a> ByondSeqSerializer<'a> {
	pub fn new(serializer: &'a mut ByondSerializer) -> Self {
		Self {
			serializer,
			list: ByondValueList::new(),
		}
	}
}

impl<'a> SerializeSeq for ByondSeqSerializer<'a> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		self.list.push(&value.serialize(&mut *self.serializer)?)?;
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.list.to_byond().map_err(SerializeError::from)
	}
}

impl<'a> SerializeTuple for ByondSeqSerializer<'a> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		self.list
			.push(&value.serialize(&mut *self.serializer)?)
			.map_err(SerializeError::from)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.list.to_byond().map_err(SerializeError::from)
	}
}

impl<'a> SerializeTupleStruct for ByondSeqSerializer<'a> {
	type Ok = ByondValue;
	type Error = SerializeError;

	fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		self.list
			.push(&value.serialize(&mut *self.serializer)?)
			.map_err(SerializeError::from)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.list.to_byond().map_err(SerializeError::from)
	}
}
