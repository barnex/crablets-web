use crate::*;

pub struct Nominal<T, N> {
	inner: T,
	_marker: std::marker::PhantomData<Mutex<N>>, // 👈 HACK: `Mutex` so that `Nominal<T, N>` can be Send+Sync even when `N` is not.
}

impl<T, N> AsRef<T> for Nominal<T, N> {
	fn as_ref(&self) -> &T {
		&self.inner
	}
}

impl<T, N> AsMut<T> for Nominal<T, N> {
	fn as_mut(&mut self) -> &mut T {
		&mut self.inner
	}
}
