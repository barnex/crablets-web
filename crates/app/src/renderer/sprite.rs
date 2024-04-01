use crate::prelude::*;

#[derive(Clone, Copy, Reflect, Serialize, Deserialize, Debug)]
pub struct Sprite(pub u16);

impl Sprite {
	pub const FERRIS_SMILE: Self = Self(0);
	pub const FERRIS: Self = Self(1);
	pub const FERRIS_UNSAFE: Self = Self(2);
	pub const FACTORY: Self = Self(24);
	pub const RECTICLE: Self = Self(32);
}
