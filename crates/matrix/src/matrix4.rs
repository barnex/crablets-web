/*
#[derive(Clone, Copy, Pod, Zeroable, Debug, Reflect, Default)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct mat4(pub [[f32; 4]; 4]);

impl mat4 {
	/// Convenience constructor, transposes its input.
	#[inline]
	pub fn transpose(el: [[f32; 4]; 4]) -> Self {
		Self::from(el).transposed()
	}

	#[inline]
	pub fn as_array(&self) -> &[f32; 16] {
		unsafe { mem::transmute(self) }
	}

	pub const UNIT: Self = Self([
		[1.0, 0.0, 0.0, 0.0], //
		[0.0, 1.0, 0.0, 0.0], //
		[0.0, 0.0, 1.0, 0.0], //
		[0.0, 0.0, 0.0, 1.0], //
	]);

	pub const ZERO: Self = Self([
		[0.0, 0.0, 0.0, 0.0], //
		[0.0, 0.0, 0.0, 0.0], //
		[0.0, 0.0, 0.0, 0.0], //
		[0.0, 0.0, 0.0, 0.0], //
	]);

	#[must_use]
	#[inline]
	pub fn transposed(&self) -> Self {
		let mut t = Self::ZERO;
		for i in 0..4 {
			for j in 0..4 {
				t.0[i][j] = self.0[j][i];
			}
		}
		t
	}

	// TODO: mul<vec4>
	pub fn transform_point_ignore_w(&self, rhs: vec3f) -> vec3f {
		let m = self.0;
		let (x, y, z) = (rhs.x(), rhs.y(), rhs.z());
		vec3f(
			m[0][0] * x + m[1][0] * y + m[2][0] * z,
			m[0][1] * x + m[1][1] * y + m[2][1] * z,
			m[0][2] * x + m[1][2] * y + m[2][2] * z,
		)
	}
}

impl From<[[f32; 4]; 4]> for mat4 {
	fn from(el: [[f32; 4]; 4]) -> Self {
		Self(el)
	}
}

impl Into<[[f32; 4]; 4]> for mat4 {
	fn into(self) -> [[f32; 4]; 4] {
		self.0
	}
}

impl Mul<vec4f> for &mat4 {
	type Output = vec4f;

	fn mul(self, rhs: vec4f) -> Self::Output {
		let (x, y, z, w) = rhs.into();
		let m = self.0;
		vec4f(
			m[0][0] * x + m[1][0] * y + m[2][0] * z + m[3][0] * w,
			m[0][1] * x + m[1][1] * y + m[2][1] * z + m[3][1] * w,
			m[0][2] * x + m[1][2] * y + m[2][2] * z + m[3][2] * w,
			m[0][3] * x + m[1][3] * y + m[2][3] * z + m[3][3] * w,
		)
	}
}

impl Mul<&mat4> for &mat4 {
	type Output = mat4;

	/// Matrix-Matrix multiplication.
	fn mul(self, rhs: &mat4) -> mat4 {
		let mut c = mat4::ZERO;
		for i in 0..4 {
			for j in 0..4 {
				for k in 0..4 {
					c.0[i][j] = c.0[i][j] + rhs.0[i][k] * self.0[k][j]
				}
			}
		}
		c
	}
}

// allows chaining multiplications:  &a * &b * &c
impl Mul<&mat4> for mat4 {
	type Output = mat4;

	/// Matrix-Matrix multiplication.
	fn mul(self, rhs: &mat4) -> mat4 {
		(&self).mul(rhs)
	}
}

impl Mul<mat4> for mat4 {
	type Output = mat4;

	/// Matrix-Matrix multiplication.
	fn mul(self, rhs: mat4) -> mat4 {
		(&self).mul(&rhs)
	}
}
*/