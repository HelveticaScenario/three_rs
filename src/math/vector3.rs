use super::matrix4::Matrix4;
use super::quaternion::Quaternion;
use super::euler::Euler;
use super::matrix3::Matrix3;
use super::super::cameras::camera::Camera;
use super::math_static::clamp;
use super::spherical::Spherical;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
	x: f32,
	y: f32,
	z: f32,
}

impl Vector3 {
	pub fn new() -> Vector3 {
		Vector3 {
			x: 0.0,
			y: 0.0,
			z: 0.0,
		}
	}

	pub fn get_x(&self) -> f32 {
		self.x
	}

	pub fn set_x(&mut self, x: f32) {
		self.x = x;
	}

	pub fn get_y(&self) -> f32 {
		self.y
	}

	pub fn set_y(&mut self, y: f32) {
		self.y = y;
	}

	pub fn get_z(&self) -> f32 {
		self.z
	}

	pub fn set_z(&mut self, z: f32) {
		self.z = z;
	}

	pub fn set(&mut self, x: f32, y: f32, z: f32) {
		self.x = x;
		self.y = y;
		self.z = z;
	}

	pub fn set_scalar(&mut self, scalar: f32) {
		self.x = scalar;
		self.y = scalar;
		self.z = scalar;
	}

	pub fn set_component(&mut self, index: i32, value: f32) {
		match index {
			0 => self.x = value,
			1 => self.y = value,
			2 => self.z = value,
			_ => panic!("index out of range: {:?}", index)
		};
	}

	pub fn get_component(&mut self, index: i32) -> f32 {
		match index {
			0 => self.x,
			1 => self.y,
			2 => self.z,
			_ => panic!("index out of range: {:?}", index)
		}
	}

	pub fn add(&mut self, v: &Vector3) {
		self.x += v.x;
		self.y += v.y;
		self.z += v.z;
	}

	pub fn add_scalar(&mut self, s: f32) {
		self.x += s;
		self.y += s;
		self.z += s;
	}

	pub fn add_vectors(&mut self, a: &Vector3, b: &Vector3) {
		self.x = a.x + b.x;
		self.y = a.y + b.y;
		self.z = a.z + b.z;
	}

	pub fn add_scaled_vector(&mut self, v: &Vector3, s: f32) {
		self.x += v.x * s;
		self.y += v.y * s;
		self.z += v.z * s;
	}

	pub fn sub(&mut self, v: &Vector3) {
		self.x -= v.x;
		self.y -= v.y;
		self.z -= v.z;
	}

	pub fn sub_scalar(&mut self, s: f32) {
		self.x -= s;
		self.y -= s;
		self.z -= s;
	}

	pub fn sub_vectors(&mut self, a: &Vector3, b: &Vector3) {
		self.x = a.x - b.x;
		self.y = a.y - b.y;
		self.z = a.z - b.z;
	}

	pub fn multiply(&mut self, v: &Vector3) {
		self.x *= v.x;
		self.y *= v.y;
		self.z *= v.z;
	}

	pub fn multiply_scalar(&mut self, scalar: f32) {
		if scalar.is_finite() {
			self.x *= scalar;
			self.y *= scalar;
			self.z *= scalar;
		} else {
			self.x = 0.0;
			self.y = 0.0;
			self.z = 0.0;
		}
	}

	pub fn multiply_vectors(&mut self, a: &Vector3, b: &Vector3) {
		self.x = a.x * b.x;
		self.y = a.y * b.y;
		self.z = a.z * b.z;
	}

	pub fn apply_euler(&mut self, euler: &Euler) {
		unimplemented!();
	}

	pub fn apply_axis_angle(&mut self) {
		unimplemented!();
	}

	pub fn apply_matrix3(&mut self, m: &Matrix3) {
		unimplemented!();
	}
	
	pub fn apply_matrix4(&mut self, m: &Matrix4) {
		unimplemented!();
	}

	pub fn apply_projection(&mut self, m: &Matrix4) {
		unimplemented!();
	}

	pub fn apply_quaternion(&mut self, m: &Quaternion) {
		unimplemented!();
	}

	pub fn project(&mut self, camera: &Camera) {
		unimplemented!();
	}

	pub fn unproject(&mut self, camera: &Camera) {
		unimplemented!();
	}

	pub fn transform_direction(&mut self, m: &Matrix4) {
		unimplemented!();
	}

	pub fn divide(&mut self, v: &Vector3) {
		self.x /= v.x;
		self.y /= v.y;
		self.z /= v.z;
	}

	pub fn divide_scalar(&mut self, scalar: f32) {
		self.multiply_scalar(1.0 / scalar);
	}

	pub fn min(&mut self, v: &Vector3) {
		self.x = self.x.min(v.x);
		self.y = self.y.min(v.y);
		self.z = self.z.min(v.z);
	}

	pub fn max(&mut self, v: &Vector3) {
		self.x = self.x.max(v.x);
		self.y = self.y.max(v.y);
		self.z = self.z.max(v.z);
	}

	pub fn clamp(&mut self, min: &Vector3, max: &Vector3) {
		self.x = min.x.max(max.x.min(self.x));
		self.y = min.y.max(max.y.min(self.y));
		self.z = min.z.max(max.z.min(self.z));
	}

	pub fn clamp_scalar(&mut self, min_val: f32, max_val: f32) {
		self.clamp(&Vector3 {
			x: min_val,
			y: min_val,
			z: min_val,
		}, &Vector3 {
			x: max_val,
			y: max_val,
			z: max_val,
		});
	}

	pub fn clamp_length(&mut self, min: f32, max: f32) {
		let length = self.length();

		self.multiply_scalar(min.max(max.min(length)) / length);
	}

	pub fn floor(&mut self) {
		self.x = self.x.floor();
		self.y = self.y.floor();
		self.z = self.z.floor();
	}

	pub fn ceil(&mut self) {
		self.x = self.x.ceil();
		self.y = self.y.ceil();
		self.z = self.z.ceil();
	}

	pub fn round(&mut self) {
		self.x = self.x.round();
		self.y = self.y.round();
		self.z = self.z.round();
	}

	pub fn round_to_zero(&mut self) {
		self.x = if self.x < 0.0 {
			self.x.ceil()
		} else {
			self.x.floor()
		};
		self.y = if self.y < 0.0 {
			self.y.ceil()
		} else {
			self.y.floor()
		};
		self.z = if self.z < 0.0 {
			self.z.ceil()
		} else {
			self.z.floor()
		};
	}

	pub fn negate(&mut self) {
		self.x = -self.x;
		self.y = -self.y;
		self.z = -self.z;
	}

	pub fn dot(&self, v: &Vector3) -> f32 {
		(self.x * v.x) + (self.y * v.y) + (self.z * v.z)
	}

	pub fn length_sq(&self) -> f32 {
		(self.x * self.x) + (self.y * self.y) + (self.z * self.z)
	}

	pub fn length(&self) -> f32 {
		self.length_sq().sqrt()
	}

	pub fn length_manhattan(&self) -> f32 {
		self.x.abs() + self.y.abs() + self.z.abs()
	}

	pub fn normalize(&mut self) {
		let length = {
			self.length()
		};
		self.divide_scalar(length)
	}

	pub fn distance_to(&self, v: &Vector3) -> f32 {
		self.distance_to_squared(v).sqrt()
	}

	pub fn distance_to_squared(&self, v: &Vector3) -> f32 {
		let dx = self.x - v.x;
		let dy = self.y - v.y;
		let dz = self.z - v.z;		
		(dx * dx) + (dy * dy) + (dz * dz)
	}

	pub fn distance_to_manhattan(&self, v: &Vector3) -> f32 {
		((self.x - v.x).abs()) + ((self.y - v.y).abs()) + ((self.z - v.z).abs())
	}

	pub fn set_length(&mut self, length: f32) {
		let l = {
			length / self.length()
		};
		self.multiply_scalar(l);
	}

	pub fn lerp(&mut self, v: &Vector3, alpha: f32) {
		self.x += (v.x - self.x) * alpha;
		self.y += (v.y - self.y) * alpha;
		self.z += (v.z - self.z) * alpha;
	}

	pub fn lerp_vectors(&mut self, v1: &Vector3, v2: &Vector3, alpha: f32) {
		self.sub_vectors(v2, v1);
		self.multiply_scalar(alpha);
		self.add(v1);
	}

	pub fn cross(&mut self, v: &Vector3) {
		let x = self.x;
		let y = self.y;
		let z = self.z;
		self.x = (y * v.z) - (z * v.y);
		self.y = (z * v.x) - (x * v.z);
		self.z = (x * v.y) - (y * v.x);
	}

	pub fn cross_vectors(&mut self, a: &Vector3, b: &Vector3) {
		let ax = a.x;
		let ay = a.y;
		let az = a.z;
		let bx = b.x;
		let by = b.y;
		let bz = b.z;
		self.x = (ay * bz) - (az * by);
		self.y = (az * bx) - (ax * bz);
		self.z = (ax * by) - (ay * bx);
	}

	pub fn project_on_vector(&mut self, vector: &Vector3) {
		let scalar = vector.dot(self) / vector.length_sq();
		self.copy(vector);
		self.multiply_scalar(scalar);
	}

	pub fn project_on_plane(&mut self, plane_normal: &Vector3) {
		let mut v1 = Vector3::new();
		v1.copy(self);
		v1.project_on_vector(plane_normal);
		self.sub(&v1);
	}

	pub fn reflect(&mut self, normal: &Vector3) {
		let mut v1 = Vector3::new();
		v1.copy(normal);
		v1.multiply_scalar(2.0 * self.dot(normal));
		self.sub(&v1);
	}

	pub fn angle_to(&self, v: &Vector3) -> f32 {
		let theta = self.dot(v) / ((self.length_sq() * v.length_sq()).sqrt());
		clamp(theta, -1.0f32, 1.0f32).acos()
	}

	pub fn set_from_spherical(&mut self, s: &Spherical) {
		unimplemented!();
	}

	pub fn set_from_matrix_position(&mut self, m: &Matrix4) {
		self.set_from_matrix_column(m, 3usize);
	}

	pub fn set_from_matrix_scale(&mut self, m: &Matrix4) {
		self.set_from_matrix_column(m, 0usize);
		let sx = self.length();
		self.set_from_matrix_column(m, 1usize);
		let sy = self.length();
		self.set_from_matrix_column(m, 2usize);
		let sz = self.length();

		self.x = sx;
		self.y = sy;
		self.z = sz;
	}

	pub fn set_from_matrix_column(&mut self, m: &Matrix4, index: usize) {
		self.copy_from_array(m.get_elements(), Some(index * 4usize));
	}

	pub fn equals(&self, v: &Vector3) -> bool {
		(v.x == self.x) && (v.y == self.y) && (v.z == self.z)
	}

	pub fn copy_from_array(&mut self, array: &[f32], offset: Option<usize>) {
		let offset = match offset {
			Some(off) => off,
			None => 0usize,
		};
		self.x = array[offset];
		self.y = array[offset + 1];
		self.z = array[offset + 2];
	}

	pub fn copy_to_array(&self, array: &mut [f32], offset: Option<usize>) {
		let offset = match offset {
			Some(off) => off,
			None => 0usize,
		};
		array[offset] = self.x;
		array[offset + 1] = self.y;
		array[offset + 2] = self.z;
	}

	pub fn copy(&mut self, v: &Vector3) {
		self.x = v.x;
		self.y = v.y;
		self.z = v.z;
	}

}
