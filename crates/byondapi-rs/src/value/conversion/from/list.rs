use crate::{
	ByondError, ByondResult, ByondTypeCheck, ByondValue, ByondValueList, ByondValueType, FromByond,
};
use std::{
	borrow::Cow,
	collections::{BTreeMap, BTreeSet, HashMap, HashSet},
	hash::{BuildHasher, Hash},
};

impl FromByond for ByondValueList {
	#[inline]
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		value.try_into()
	}
}

impl<Value> FromByond for Vec<Value>
where
	Value: FromByond,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		let val = value.get_type();
		match val {
			ByondValueType::LIST => (),
			ByondValueType::NULL => {
				return Ok(Vec::new());
			}
			_ => {
				return Err(ByondError::InvalidConversion {
					expected: Cow::Borrowed("list"),
					got: val.name(),
				});
			}
		}
		value
			.iter()?
			.map(|(key, value)| Value::from_byond(&value.unwrap_or(key)))
			.collect::<ByondResult<Self>>()
	}
}

impl<Value, Hasher> FromByond for HashSet<Value, Hasher>
where
	Value: FromByond + Hash + Eq,
	Hasher: BuildHasher + Default,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		let val = value.get_type();
		match val {
			ByondValueType::LIST => (),
			ByondValueType::NULL => {
				return Ok(Self::default());
			}
			_ => {
				return Err(ByondError::InvalidConversion {
					expected: Cow::Borrowed("list"),
					got: val.name(),
				})
			}
		}
		value
			.iter()?
			.map(|(key, _)| Value::from_byond(&key))
			.collect::<ByondResult<Self>>()
	}
}

impl<Value> FromByond for BTreeSet<Value>
where
	Value: FromByond + Ord,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		let val = value.get_type();
		match val {
			ByondValueType::LIST => (),
			ByondValueType::NULL => return Ok(Self::default()),
			_ => {
				return Err(ByondError::InvalidConversion {
					expected: Cow::Borrowed("list"),
					got: val.name(),
				})
			}
		}
		value
			.iter()?
			.map(|(key, _)| Value::from_byond(&key))
			.collect::<ByondResult<Self>>()
	}
}

impl<Key, Value, Hasher> FromByond for HashMap<Key, Value, Hasher>
where
	Key: FromByond + Hash + Eq,
	Value: FromByond,
	Hasher: BuildHasher + Default,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		let val = value.get_type();
		match val {
			ByondValueType::LIST => (),
			ByondValueType::NULL => return Ok(Self::default()),
			_ => {
				return Err(ByondError::InvalidConversion {
					expected: Cow::Borrowed("list"),
					got: val.name(),
				})
			}
		}
		value
			.iter()?
			.map(|(key, value)| {
				Ok((
					Key::from_byond(&key)?,
					Value::from_byond(&value.unwrap_or(key))?,
				))
			})
			.collect::<ByondResult<Self>>()
	}
}

impl<Key, Value> FromByond for BTreeMap<Key, Value>
where
	Key: FromByond + Ord,
	Value: FromByond,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		let val = value.get_type();
		match val {
			ByondValueType::LIST => (),
			ByondValueType::NULL => return Ok(Self::default()),
			_ => {
				return Err(ByondError::InvalidConversion {
					expected: Cow::Borrowed("list"),
					got: val.name(),
				})
			}
		}
		value
			.iter()?
			.map(|(key, value)| {
				Ok((
					Key::from_byond(&key)?,
					Value::from_byond(&value.unwrap_or(key))?,
				))
			})
			.collect::<ByondResult<Self>>()
	}
}
