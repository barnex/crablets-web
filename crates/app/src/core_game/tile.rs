use crate::prelude::*;

#[derive(Clone, Copy, Reflect, Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Tile(pub u8);

#[allow(unused)]
impl Tile{
	pub const WATER: Tile = Tile(0);
	pub const FARM_LAND: Tile = Tile(1);
	pub const STONE: Tile = Tile(2);
}