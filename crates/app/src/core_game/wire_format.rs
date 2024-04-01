use crate::prelude::*;

pub fn serialize_file<T: Serialize>(path: impl AsRef<Path>, t: &T) -> Result<()> {
	let path = path.as_ref();
	serialize(&mut create(path)?, t).with_context(|| anyhow!("serialize {path:?}"))
}

pub fn serialize<T: Serialize, W: Write>(w: &mut W, t: &T) -> Result<()> {
	t.serialize(&mut rmp_serde::Serializer::new(w).with_struct_map()).context("serialize message pack")
}

pub fn deserialize_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
	let path = path.as_ref();
	deserialize(&mut open(path)?).with_context(|| anyhow!("deserialize {path:?}"))
}

pub fn deserialize<T: DeserializeOwned, R: Read>(r: &mut R) -> Result<T> {
	T::deserialize(&mut rmp_serde::Deserializer::new(r)).context("deserialize message pack")
}
