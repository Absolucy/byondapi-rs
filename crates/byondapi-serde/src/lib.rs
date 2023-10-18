pub mod de;
pub mod error;
pub mod ser;

use byondapi::ByondValue;

#[inline]
pub fn serialize<Value>(v: &Value) -> Result<ByondValue, error::SerializeError>
where
	Value: serde::Serialize,
{
	v.serialize(&mut ser::ByondSerializer)
}

#[inline]
pub fn deserialize<'de, Value>(value: ByondValue) -> Result<Value, error::DeserializeError>
where
	Value: serde::Deserialize<'de>,
{
	Value::deserialize(&de::ByondDeserializer { value })
}
