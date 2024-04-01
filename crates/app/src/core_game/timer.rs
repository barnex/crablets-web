use crate::*;

#[derive(Clone, Reflect, Serialize, Deserialize, Default)]
pub struct Timer {
	pub alarm: u64,
}

impl Timer {
	pub fn new(alarm: u64) -> Self {
		Self { alarm }
	}

	pub fn set_alarm(&mut self, now: u64, duration: u64) {
		self.alarm = now + duration
	}

	#[must_use]
	pub fn just_finished(&self, now: u64) -> bool {
		now == self.alarm
	}

	#[must_use]
	pub fn finished(&self, now: u64) -> bool {
		now >= self.alarm
	}
}
