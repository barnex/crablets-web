use crate::*;

pub const RESOURCE_RESERVATION_TTL_MAJOR_TICKS: u8 = 128; // TODO: distance-based
pub const FACTORY_RESERVATION_TTL_MAJOR_TICKS: u64 = 512; //

impl GameState {
	pub fn tick_resource_reservation(&mut self) {
		if self.now % MAJOR_TICK == 0 {
			for v in &mut self.resource_reservations.values {
				v.saturating_dec()
			}

			if self.now % FACTORY_RESERVATION_TTL_MAJOR_TICKS == 0 {
				for buf in self.factories.iter_mut().flat_map(|fac| fac.outputs.iter_mut()) {
					buf.unreserve_item()
				}
				for buf in self.factories.iter_mut().flat_map(|fac| fac.inputs.iter_mut()) {
					buf.unreserve_slot()
				}
			}
		}
	}

	pub fn draw_resource_reservation(&self, sg: &mut Scenegraph) {
		sg.new_layer(); // 👈

		let sprite = Sprite::RECTICLE;

		for pos in self.visible_tile_range().iter_excl() {
			if self.resource_reservations.at_or_default(pos) != 0 {
				sg.instances.push(InstanceData::new(pos.center(), sprite));
			}
		}
	}

	pub fn reserve_resource(&mut self, pos: vec2i) -> bool {
		if let Some(v) = self.resource_reservations.try_at_mut(pos) {
			if *v != 0 {
				return false;
			}
			*v = RESOURCE_RESERVATION_TTL_MAJOR_TICKS;
			true
		} else {
			false
		}
	}
}
