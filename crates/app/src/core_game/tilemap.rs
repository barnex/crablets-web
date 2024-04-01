use crate::prelude::*;

#[derive(Clone, Reflect, Serialize, Deserialize)]
pub struct Tilemap {
	pub tiles: Vec2D<Tile>,

	// maps tile -> sprite for rendering
	pub sprites: Vec<Sprite>,
}

impl Tilemap {
	pub fn new(size: vec2u, fill: Tile) -> Self {
		Self {
			sprites: offset_mapping(),
			tiles: Vec2D::new(size, fill),
		}
	}

	pub fn at(&self, index: vec2u) -> Tile {
		*self.tiles.at(index)
	}

	pub fn try_at(&self, index: vec2i) -> Option<Tile> {
		self.tiles.try_at(index).copied()
	}

	pub fn at_mut(&mut self, index: vec2u) -> &mut Tile {
		self.tiles.at_mut(index)
	}

	pub fn try_set(&mut self, index: vec2i, value: Tile) {
		self.tiles.try_set(index, value)
	}

	pub fn bounds(&self) -> Bounds2u {
		self.tiles.bounds()
	}

	pub fn iter_range_excl(&self, bounds: Bounds2i) -> impl Iterator<Item = (vec2i, Tile)> + '_ {
		self.tiles.iter_range_excl(bounds)
	}

	pub fn iter(&self) -> impl Iterator<Item = (vec2u, Tile)> + '_ {
		self.tiles.iter().map(|(pos, tile)| (pos, *tile))
	}
}


fn unit_mapping() -> Vec<Sprite> {
	(0..=255).map(|i| Sprite(i as u16)).collect()
}

fn offset_mapping() -> Vec<Sprite> {
	const OFF: i16 = 8;
	(0..=255).map(|i| Sprite((i + OFF) as u16)).collect()
}
