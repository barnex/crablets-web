use crate::*;

/*
#[derive(Reflect, Serialize, Deserialize)]
pub struct List<T> {
	_curr_version: u32,
	values: Vec<T>,
	versions: Vec<u32>,
	free_indices: Vec<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ID {
	index: u32,
	version: u32,
}

impl fmt::Debug for ID {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}_{}", self.index, self.version)
	}
}

impl<T> List<T> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_capacity(cap: usize) -> Self {
		Self {
			_curr_version: default(),
			values: Vec::with_capacity(cap),
			versions: Vec::with_capacity(cap),
			free_indices: default(),
		}
	}

	pub fn insert(&mut self, v: T) -> ID {
		let version = self.new_version();
		if let Some(index) = self.free_indices.pop() {
			self.values[index as usize] = v;
			self.versions[index as usize] = version;
			ID { index, version }
		} else {
			let index = self.values.len() as u32;
			self.values.push(v);
			self.versions.push(version);
			ID { index, version }
		}
	}

	pub fn get(&self, id: ID) -> Option<&T> {
		let version = *self.versions.get(id.index as usize)?;
		if version == id.version {
			Some(&self.values[id.index as usize])
		} else {
			None
		}
	}

	pub fn get_mut(&mut self, id: ID) -> Option<&mut T> {
		let version = *self.versions.get(id.index as usize)?;
		if version == id.version {
			Some(&mut self.values[id.index as usize])
		} else {
			None
		}
	}

	pub fn remove(&mut self, id: ID) -> Option<&T> {
		let version = *self.versions.get(id.index as usize)?;
		if version == id.version {
			self.versions[id.index as usize] = 0;
			self.free_indices.push(id.index);
			Some(&self.values[id.index as usize])
		} else {
			None
		}
	}

	pub fn ids(&self) -> impl Iterator<Item = ID> + '_ {
		self.versions
			.iter()
			.enumerate()
			.filter_map(|(index, &version)| (version != 0).then_some(ID { index: index as u32, version }))
	}

	pub fn enumerate(&self) -> impl Iterator<Item = (ID, &T)> {
		self.values.iter().enumerate().zip(&self.versions).filter_map(|((index, value), version)| {
			(*version != 0).then_some((
				ID {
					index: index as u32,
					version: *version,
				},
				value,
			))
		})
	}

	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.values.iter().zip(&self.versions).filter_map(|(value, version)| (*version != 0).then_some(value))
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.values.iter_mut().zip(&self.versions).filter_map(|(value, version)| (*version != 0).then_some(value))
	}

	fn new_version(&mut self) -> u32 {
		self._curr_version += 1;
		self._curr_version
	}
}

impl<T> Default for List<T> {
	fn default() -> Self {
		Self::with_capacity(0)
	}
}

impl<T> FromIterator<T> for List<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let iter = iter.into_iter();
		let mut list = List::with_capacity(iter.size_hint().0);
		for v in iter {
			list.insert(v);
		}
		list
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_list() {
		let mut list = List::<&'static str>::new();

		let bob_id = list.insert("bob");
		let alice_id = list.insert("alice");

		assert_eq!(list.ids().collect_vec(), vec![bob_id, alice_id]);
		assert_eq!(list.iter().collect_vec(), vec![&"bob", &"alice"]);

		assert_eq!(list.get(bob_id), Some(&"bob"));
		assert_eq!(list.get(alice_id), Some(&"alice"));

		assert_eq!(list.remove(bob_id), Some(&"bob"));

		assert_eq!(list.get(bob_id), None);
		assert_eq!(list.get(alice_id), Some(&"alice"));

		let foo_id = list.insert("foo");
		assert_eq!(list.get(bob_id), None);
		assert_eq!(list.get(foo_id), Some(&"foo"));

		assert_eq!(list.ids().collect_vec(), vec![foo_id, alice_id]);
		assert_eq!(list.iter().collect_vec(), vec![&"foo", &"alice"]);
	}
}
*/