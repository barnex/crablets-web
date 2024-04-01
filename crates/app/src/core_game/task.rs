use crate::*;

#[derive(Reflect, Serialize, Deserialize, Clone)]
pub enum Task {
	Harvest { resource: Resource, from: vec2i, to: vec2i },
	FactoryPickUp { resource: Resource, from: vec2i, to: vec2i },
	FactoryDeliver { to: vec2i },
	None,
}

impl Task {
	pub fn is_none(&self) -> bool {
		matches!(self, Self::None)
	}

	pub fn is_some(&self) -> bool {
		!self.is_none()
	}
}
