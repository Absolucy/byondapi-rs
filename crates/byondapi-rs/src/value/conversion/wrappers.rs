use crate::{ByondError, ByondResult, ByondTypeCheck, ByondValue, FromByond, ToByond};
use std::{
	fmt::{Debug, Display},
	hash::Hash,
	ops::{Deref, DerefMut},
	str::FromStr,
};

#[must_use]
#[repr(transparent)]
pub struct StringWrapper<Value>(Value);

impl<Value> StringWrapper<Value> {
	#[inline]
	pub fn into_inner(self) -> Value {
		self.0
	}
}

impl<Value> Deref for StringWrapper<Value> {
	type Target = Value;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<Value> DerefMut for StringWrapper<Value> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<Value> AsRef<Value> for StringWrapper<Value> {
	#[inline]
	fn as_ref(&self) -> &Value {
		&self.0
	}
}

impl<Value> AsMut<Value> for StringWrapper<Value> {
	#[inline]
	fn as_mut(&mut self) -> &mut Value {
		&mut self.0
	}
}

impl<Value> From<Value> for StringWrapper<Value> {
	#[inline]
	fn from(value: Value) -> Self {
		Self(value)
	}
}

impl<Value> Clone for StringWrapper<Value>
where
	Value: Clone,
{
	#[inline]
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<Value> Copy for StringWrapper<Value> where Value: Copy {}

impl<Value> PartialEq for StringWrapper<Value>
where
	Value: PartialEq,
{
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		self.0.eq(&other.0)
	}
}

impl<Value> Eq for StringWrapper<Value> where Value: Eq {}

impl<Value> PartialOrd for StringWrapper<Value>
where
	Value: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.0.partial_cmp(&other.0)
	}
}

impl<Value> Ord for StringWrapper<Value>
where
	Value: Ord,
{
	#[inline]
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.cmp(&other.0)
	}
}

impl<Value> Debug for StringWrapper<Value>
where
	Value: Debug,
{
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl<Value> Display for StringWrapper<Value>
where
	Value: Display,
{
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl<Value> FromStr for StringWrapper<Value>
where
	Value: FromStr,
{
	type Err = Value::Err;

	#[inline]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Value::from_str(s).map(Self)
	}
}

impl<Value> Hash for StringWrapper<Value>
where
	Value: Hash,
{
	#[inline]
	fn hash<Hasher>(&self, state: &mut Hasher)
	where
		Hasher: std::hash::Hasher,
	{
		self.0.hash(state)
	}
}

impl<Value> Default for StringWrapper<Value>
where
	Value: Default,
{
	#[inline]
	fn default() -> Self {
		Self(Value::default())
	}
}

impl<Value> FromByond for StringWrapper<Value>
where
	Value: FromByond + FromStr,
	Value::Err: std::error::Error + 'static,
{
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		if value.is_str() {
			String::from_byond(value)
				.and_then(|string| Value::from_str(&string).map_err(ByondError::boxed))
				.map(Self)
		} else {
			Value::from_byond(value).map(Self)
		}
	}
}

impl<Value> ToByond for StringWrapper<Value>
where
	Value: FromByond + ToString,
{
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.to_string().to_byond()
	}
}
