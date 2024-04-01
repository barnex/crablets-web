use crate::prelude::*;

pub fn tick_grow_crops(gs: &mut GameState) {
	if gs.now % MAJOR_TICK == 0 {
		randomly_grow_crops(gs, gs.grow_crops_rate)
	}
}

pub fn instantly_grow_crops(gs: &mut GameState) {
	randomly_grow_crops(gs, 1.0)
}

fn randomly_grow_crops(gs: &mut GameState, chance: f32) {
	let mut rng = rand::thread_rng();
	let tilemap = &gs.tilemap;
	for (pos, _tile) in tilemap.iter().filter(|(_, tile)| tile == &Tile::WATER) {
		let pos = pos.as_i32();
		for (dx, dy) in (-1..=1).cartesian_product(-1..=1) {
			let pos = pos + vec2i(dx, dy);
			if tilemap.tiles.at_or_default(pos) == Tile::FARM_LAND {
				let pos = pos.as_u32();
				if gs.resources.at(pos).is_none() && rng.gen::<f32>() < chance {
					*gs.resources.at_mut(pos) = Some(Resource::SEAWEED);
				}
			}
		}
	}
}
