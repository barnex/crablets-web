#![allow(non_camel_case_types)]

use bevy_reflect::Reflect;
use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Debug, Reflect)]
#[repr(C)]
pub struct mat<T, const N: usize>(pub [[T; N]; N]);

pub type mat2x2<T> = mat<T, 2>;
pub type mat3x3<T> = mat<T, 3>;
pub type mat4x4<T> = mat<T, 4>;

pub type mat2x2f = mat<f32, 2>;
pub type mat3x3f = mat<f32, 3>;
pub type mat4x4f = mat<f32, 4>;

unsafe impl<T, const N: usize> Zeroable for mat<T, N> where T: Zeroable {}
unsafe impl<T, const N: usize> Pod for mat<T, N> where T: Pod {}

impl<T, const N: usize> Default for mat<T, N>
where
	[[T; N]; N]: Default,
{
	fn default() -> Self {
		Self(<[[T; N]; N]>::default())
	}
}

impl<T, const N: usize> mat<T, N> where [T; N]: Default {}

/*
mod matrix3;
mod matrix4;
mod transforms;

pub use matrix3::*;
pub use matrix4::*;
pub use transforms::*;
*/
