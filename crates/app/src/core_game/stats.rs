use crate::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Serialize, Deserialize, Default, Reflect)]
pub struct Stats {
	pub frame: [u64; Event::_Num as usize],
	pub total: [u64; Event::_Num as usize],
}

impl Stats {
	pub fn iter(&self) -> impl Iterator<Item = (Event, u64, u64)> + '_ {
		Event::all().map(|stat| (stat, self.frame[stat as usize], self.total[stat as usize]))
	}
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum Event {
	CallAnimateCrab = 0,
	CallNavigateCrab = 1,
	NavigationStuck = 2,
	CallRelax = 3,
	GiveWorkOk = 4,
	GiveWorkErr = 5,
	PickUpOk = 6,
	PickUpErr = 7,
	DropOffOk = 8,
	DropOffErr = 9,
	_Num = 10, // used to determine array size. Must be last.
}

impl Event {
	pub fn all() -> impl Iterator<Item = Event> {
		(0..(Event::_Num as usize)).map(|i| Event::from_usize(i).unwrap())
	}
}

impl Stats {
	pub fn inc(&mut self, stat: Event) {
		self.frame[stat as usize] += 1;
	}

	pub fn next_frame(&mut self) {
		for (frame, total) in iter::zip(&mut self.frame, &mut self.total) {
			*total += *frame;
			*frame = 0;
		}
	}
}

impl fmt::Display for Stats {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for i in 0..(Event::_Num as usize) {
			let stat = Event::from_u8(i as u8).expect("Stats: bug");
			writeln!(f, "{:?}: {}, {}", stat, self.frame[i], self.total[i])?;
		}
		Ok(())
	}
}
