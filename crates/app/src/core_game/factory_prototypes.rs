use crate::*;

pub fn building_site_prototype(factory_prototype: usize) -> Factory {
	let prototype = FACTORY_PROTOTYPES[factory_prototype].1.clone();
	Factory {
		inputs: Vec::from([Input::new(Resource::BRICK, 24)]),
		outputs: Vec::new(),
		priority: 100,
		logic: FactoryLogic::BuildingSite { factory_prototype },
		..prototype
	}
}

pub fn dig_site_prototype(tile: Tile) -> Factory {
	Factory {
		inputs: Vec::from([Input::new(Resource::BRICK, 6)]),
		outputs: Vec::new(),
		priority: 100,
		logic: FactoryLogic::DigSite { tile },
		position: default(),
		scale: 1,
		sprite: Sprite::RECTICLE, // << TODO: use tile sprite, requires fixed tilemap?
	}
}

pub static FACTORY_PROTOTYPES: LazyLock<[(&str, Factory); 5]> = LazyLock::new(|| {
	[
		(
			"rock pile",
			Factory {
				position: vec::ZERO,
				scale: 2,
				sprite: Sprite::FACTORY,
				inputs: Vec::from([Input::new(Resource::ROCK, 20)]),
				outputs: Vec::from([Output::new(Resource::ROCK, 20)]),
				priority: 10,
				logic: default(),
			},
		),
		(
			"seaweed pile",
			Factory {
				position: vec::ZERO,
				scale: 2,
				sprite: Sprite::FACTORY,
				inputs: Vec::from([Input::new(Resource::SEAWEED, 20)]),
				outputs: Vec::from([Output::new(Resource::SEAWEED, 20)]),
				priority: 20,
				logic: default(),
			},
		),
		(
			"brick pile",
			Factory {
				position: vec::ZERO,
				scale: 2,
				sprite: Sprite::FACTORY,
				inputs: Vec::from([Input::new(Resource::BRICK, 20)]),
				outputs: Vec::from([Output::new(Resource::BRICK, 20)]),
				priority: 30,
				logic: default(),
			},
		),
		(
			"brick oven",
			Factory {
				position: vec::ZERO,
				scale: 3,
				sprite: Sprite::FACTORY,
				inputs: Vec::from([Input::new(Resource::ROCK, 12), Input::new(Resource::DRYWEED, 15)]),
				outputs: Vec::from([Output::new(Resource::BRICK, 5)]),
				priority: 30,
				logic: default(),
			},
		),
		(
			"seaweed dryery",
			Factory {
				position: vec::ZERO,
				scale: 2,
				sprite: Sprite::FACTORY,
				inputs: vec![Input::new(Resource::SEAWEED, 18)],
				outputs: vec![Output::new(Resource::DRYWEED, 6)],
				priority: 30,
				logic: default(),
			},
		),
	]
});
