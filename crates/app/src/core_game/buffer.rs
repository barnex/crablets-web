#![allow(clippy::unit_arg)]
use crate::*;

use core_game::Error;
use core_game::Result;

#[derive(Serialize, Deserialize, Reflect, Clone, Debug)]
pub struct Input {
	resource: Resource,
	capacity: u16,
	num_items: u16,
	reserved_slots: u16,
}

#[derive(Serialize, Deserialize, Reflect, Clone, Debug)]
pub struct Output {
	resource: Resource,
	capacity: u16,
	num_items: u16,
	reserved_items: u16,
}

impl Input {
	pub const fn new(resource: Resource, capacity: u16) -> Self {
		Self {
			resource,
			capacity,
			num_items: 0,
			reserved_slots: 0,
		}
	}

	pub fn resource(&self) -> Resource {
		self.resource
	}

	pub fn is_empty(&self) -> bool {
		self.num_items == 0
	}

	pub fn is_virtually_full(&self) -> bool {
		self.num_items + self.reserved_slots >= self.capacity
	}

	pub fn is_physically_full(&self) -> bool {
		self.num_items >= self.capacity
	}

	pub fn reserve_slot(&mut self) -> Result<()> {
		match self.is_virtually_full() {
			true => Err(Error::Full),
			false => Ok(self.reserved_slots.replace_with(|v| (v + 1).clamp(0, self.capacity))),
		}
	}

	pub fn unreserve_slot(&mut self) {
		self.reserved_slots.saturating_dec()
	}

	pub fn drain(&mut self) {
		self.num_items = 0;
		self.reserved_slots = 0;
	}

	pub fn add_to(&mut self) -> Result<()> {
		match self.is_physically_full() {
			true => Err(Error::Full),
			false => {
				self.num_items += 1;
				self.reserved_slots.saturating_dec();
				Ok(())
			}
		}
	}

	pub fn consume_items(&mut self, n: u16) -> Result<()> {
		match self.num_items >= n {
			true => Ok(self.num_items -= n),
			false => Err(Error::Empty),
		}
	}

	pub fn draw_on(&self, sg: &mut Scenegraph, position: vec2f, scale: f32) {
		match self.num_items {
			0 => sg.instances.push(InstanceData::new(position, self.resource.sprite()).mix_color(vec4(0.0, 0.0, 0.0, 0.5))),
			n => {
				for i in 0..n {
					let pos_y = scale * (i as f32) / (self.capacity as f32);
					sg.instances.push(InstanceData::new(position + vec2(0.0, pos_y), self.resource.sprite()))
				}
			}
		}

		// TODO: if reservation overlay...
		for i in self.num_items..(self.num_items + self.reserved_slots) {
			let pos_y = scale * (i as f32) / (self.capacity as f32);
			sg.instances.push(InstanceData::new(position + vec2(0.0, pos_y), Sprite::RECTICLE))
		}
	}
}

impl Output {
	pub const fn new(resource: Resource, capacity: u16) -> Self {
		Self {
			resource,
			capacity,
			num_items: 0,
			reserved_items: 0,
		}
	}

	pub fn resource(&self) -> Resource {
		self.resource
	}

	pub fn is_full(&self) -> bool {
		self.num_items >= self.capacity
	}

	pub fn is_vitrually_empty(&self) -> bool {
		self.num_items as i32 - self.reserved_items as i32 <= 0
	}

	pub fn is_physically_empty(&self) -> bool {
		self.num_items == 0
	}

	pub fn reserve_item(&mut self) -> Result<()> {
		match self.is_vitrually_empty() {
			true => Err(Error::Empty),
			false => Ok(self.reserved_items.replace_with(|v| (v + 1).clamp(0, self.capacity))),
		}
	}

	pub fn unreserve_item(&mut self) {
		self.reserved_items.saturating_dec()
	}

	pub fn take_item(&mut self) -> Result<()> {
		match self.is_physically_empty() {
			true => Err(Error::Empty),
			false => {
				self.num_items -= 1;
				self.reserved_items.saturating_dec();
				Ok(())
			}
		}
	}

	pub fn produce_items(&mut self, n: u16) -> Result<()> {
		match self.num_items + n <= self.capacity {
			true => Ok(self.num_items += n),
			false => Err(Error::Full),
		}
	}

	pub fn drain(&mut self) {
		self.num_items = 0;
		self.reserved_items = 0;
	}

	pub fn draw_on(&self, sg: &mut Scenegraph, position: vec2f, scale: f32) {
		// TODO: factor
		match self.num_items {
			0 => sg.instances.push(InstanceData::new(position, self.resource.sprite()).mix_color(vec4(0.0, 0.0, 0.0, 0.5))),
			n => {
				for i in 0..n {
					let pos_y = scale * (i as f32) / (self.capacity as f32);
					sg.instances.push(InstanceData::new(position + vec2(0.0, pos_y), self.resource.sprite()))
				}
			}
		}

		// TODO: if reservation overlay...
		for i in 0..self.reserved_items {
			let pos_y = scale * (i as f32) / (self.capacity as f32);
			sg.instances.push(InstanceData::new(position + vec2(0.0, pos_y), Sprite::RECTICLE))
		}
	}
}
