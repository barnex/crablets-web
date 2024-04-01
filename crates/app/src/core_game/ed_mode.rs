use crate::*;

#[derive(Serialize, Deserialize, Reflect, Debug, PartialEq, Eq, Clone, Copy)]
pub enum EdMode {
	Select,
	Pencil,
	Buildings,
	Fill,
}

impl Default for EdMode {
	fn default() -> Self {
		Self::Select
	}
}
