use crate::prelude::*;

/// 2D Array.
#[derive(Clone, Reflect, Serialize, Deserialize)]
pub struct Vec2D<T> {
	pub size: vec2u,
	pub values: Vec<T>,
}

impl<T> Vec2D<T>
where
	T: Clone,
{
	pub fn new(size: vec2u, fill: T) -> Self {
		Self {
			size,
			values: (0..(size.x() * size.y())).map(|_| fill.clone()).collect(),
		}
	}
}

impl<T> Vec2D<T>
where
	T: Clone + Default,
{
	pub fn clear(&mut self) {
		let default = T::default();
		self.values.iter_mut().for_each(|v| *v = default.clone())
	}

	pub fn at_or_default(&self, pos: vec2i) -> T {
		self.try_at(pos).cloned().unwrap_or_default()
	}

	pub fn iter_range_excl(&self, bounds: Bounds2i) -> impl Iterator<Item = (vec2i, T)> + '_ {
		// TODO: too many checks and conversions
		bounds
			.intersect(self.bounds().as_i32())
			.as_u32()
			.iter_excl()
			.map(|pos| (pos.as_i32(), self.at_or_default(pos.as_i32())))
	}
}

impl<T> Vec2D<T> {
	pub fn at(&self, pos: vec2u) -> &T {
		#[cfg(debug_assertions)]
		if !(pos.x() < self.size.x() && pos.y() < self.size.y()) {
			panic!("Vec2D::at({pos}): out of range {:?}", self.bounds())
		}
		&self.values[(pos.x() + self.size.x() * pos.y()) as usize]
	}

	pub fn try_at(&self, pos: vec2i) -> Option<&T> {
		self.bounds().as_i32().contains(pos).then(|| self.at(pos.as_u32()))
	}

	pub fn at_mut(&mut self, pos: vec2u) -> &mut T {
		debug_assert!(pos.x() < self.size.x() && pos.y() < self.size.y());
		&mut self.values[(pos.x() + self.size.x() * pos.y()) as usize]
	}

	pub fn try_at_mut(&mut self, pos: vec2i) -> Option<&mut T> {
		self.bounds().as_i32().contains(pos).then(|| self.at_mut(pos.as_u32()))
	}

	pub fn try_set(&mut self, pos: vec2i, value: T) {
		if self.bounds().as_i32().contains(pos) {
			*self.at_mut(pos.as_u32()) = value;
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = (vec2u, &T)> + '_ {
		//self.iter_positions().enumerate().map(|(i, pos)| (pos, &self.values[i]))
		self.iter_positions().map(|pos| (pos, self.at(pos)))
	}

	pub fn iter_positions(&self) -> impl Iterator<Item = vec2u> {
		(0..self.size.y()).cartesian_product(0..self.size.x()).map(|(x, y)| vec2(y, x)) // 👈 must transpose so that x is inner loop
	}
}

impl<T> Vec2D<Option<T>> {
	pub fn take_at(&mut self, pos: vec2i) -> Option<T> {
		match self.try_at_mut(pos) {
			None => None,
			Some(ptr) => Option::take(ptr),
		}
	}
}

impl<T> Vec2D<T> {
	pub fn bounds(&self) -> Bounds2u {
		Bounds { min: vec::ZERO, max: self.size }
	}
}
