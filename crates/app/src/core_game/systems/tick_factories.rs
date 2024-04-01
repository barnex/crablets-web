use crate::*;

impl GameState {
	pub fn tick_factories(&mut self) {
		if !self.debug.tick_factories {
			return;
		}

		for i in 0..self.factories.len() {
			let mut mutable_clone = self.factories[i].clone();
			mutable_clone.tick(self);
			self.factories[i] = mutable_clone;
		}

		if self.debug.expire_factory_reservations {
			self.tick_expire_reservations();
		}
	}

	fn tick_expire_reservations(&mut self) {
		if self.now % (32 * MAJOR_TICK) == 0 {
			for input in self.factories.iter_mut().flat_map(|fac| fac.inputs.iter_mut()) {
				input.unreserve_slot()
			}
			for output in self.factories.iter_mut().flat_map(|fac| fac.outputs.iter_mut()) {
				output.unreserve_item()
			}
		}
	}
}
