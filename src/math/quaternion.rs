use super::euler::{Euler, RotationOrders};
use super::vector3::Vector3;
use super::matrix4::Matrix4;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
	x: f32,
	y: f32,
	z: f32,
	w: f32,
}

impl Quaternion {
	pub fn new() -> Quaternion {
		Quaternion{
			x: 0.0f32,
			y: 0.0f32,
			z: 0.0f32,
			w: 1.0f32,
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

	pub fn get_w(&self) -> f32 {
		self.w
	}
	
	pub fn set_w(&mut self, w: f32) {
		self.w = w;
	}
	
	pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
		self.x = x;
		self.y = y;
		self.z = z;
		self.w = w;
	}

	pub fn copy(&mut self, quaternion: &Quaternion) {
		self.x = quaternion.x;
		self.y = quaternion.y;
		self.z = quaternion.z;
		self.w = quaternion.w;
	}

	pub fn set_from_euler(&mut self, euler: &Euler, update: bool) {

		let c1 = ( euler.get_x() / 2.0f32 ).cos();
		let c2 = ( euler.get_y() / 2.0f32 ).cos();
		let c3 = ( euler.get_z() / 2.0f32 ).cos();
		let s1 = ( euler.get_x() / 2.0f32 ).sin();
		let s2 = ( euler.get_y() / 2.0f32 ).sin();
		let s3 = ( euler.get_z() / 2.0f32 ).sin();

		let order = euler.get_order();

		match order {
			RotationOrders::XYZ => {
				self.x = (s1 * c2 * c3) + (c1 * s2 * s3);
				self.y = (c1 * s2 * c3) - (s1 * c2 * s3);
				self.z = (c1 * c2 * s3) + (s1 * s2 * c3);
				self.w = (c1 * c2 * c3) - (s1 * s2 * s3);	
			},
			RotationOrders::YXZ => {
				self.x = (s1 * c2 * c3) + (c1 * s2 * s3);
				self.y = (c1 * s2 * c3) - (s1 * c2 * s3);
				self.z = (c1 * c2 * s3) - (s1 * s2 * c3);
				self.w = (c1 * c2 * c3) + (s1 * s2 * s3);
			},
			RotationOrders::ZXY => {
				self.x = (s1 * c2 * c3) - (c1 * s2 * s3);
				self.y = (c1 * s2 * c3) + (s1 * c2 * s3);
				self.z = (c1 * c2 * s3) + (s1 * s2 * c3);
				self.w = (c1 * c2 * c3) - (s1 * s2 * s3);
			},
			RotationOrders::ZYX => {
				self.x = (s1 * c2 * c3) - (c1 * s2 * s3);
				self.y = (c1 * s2 * c3) + (s1 * c2 * s3);
				self.z = (c1 * c2 * s3) - (s1 * s2 * c3);
				self.w = (c1 * c2 * c3) + (s1 * s2 * s3);
			},
			RotationOrders::YZX => {
				self.x = (s1 * c2 * c3) + (c1 * s2 * s3);
				self.y = (c1 * s2 * c3) + (s1 * c2 * s3);
				self.z = (c1 * c2 * s3) - (s1 * s2 * c3);
				self.w = (c1 * c2 * c3) - (s1 * s2 * s3);
			},
			RotationOrders::XZY => {
				self.x = (s1 * c2 * c3) - (c1 * s2 * s3);
				self.y = (c1 * s2 * c3) - (s1 * c2 * s3);
				self.z = (c1 * c2 * s3) + (s1 * s2 * c3);
				self.w = (c1 * c2 * c3) + (s1 * s2 * s3);
			}
		}

	}

	pub fn set_from_axis_angle(&mut self, axis: &Vector3, angle: f32) {
		let half_angle = angle / 2.0f32;
		let s = half_angle.sin();

		self.x = axis.get_x() * s;
		self.y = axis.get_y() * s;
		self.z = axis.get_z() * s;
		self.w = half_angle.cos();
	}

	pub fn set_from_rotation_matrix(&mut self, m: &Matrix4) {
		let te = m.get_elements();
		let m11 = te[0];
		let m12 = te[4];
		let m13 = te[8];
		let m21 = te[1];
		let m22 = te[5];
		let m23 = te[9];
		let m31 = te[2];
		let m32 = te[6];
		let m33 = te[10];
		let trace = m11 + m22 + m33;
		if trace > 0.0f32 {
			let s = 0.5f32 / (trace + 1.0f32).sqrt();
			self.w = 0.25f32 / s;
			self.x = ( m32 - m23 ) * s;
			self.y = ( m13 - m31 ) * s;
			self.z = ( m21 - m12 ) * s;
		} else if m11 > m22 && m11 > m33 {
			let s = 2.0f32 * (1.0f32 + m11 - m22 - m33).sqrt();

			self.w = ( m32 - m23 ) / s;
			self.x = 0.25f32 * s;
			self.y = ( m12 + m21 ) / s;
			self.z = ( m13 + m31 ) / s;
		} else if m22 > m33 {
			let s = 2.0f32 * (1.0f32 + m22 - m11 - m33);
			self.w = ( m13 - m31 ) / s;
			self.x = ( m12 + m21 ) / s;
			self.y = 0.25f32 * s;
			self.z = ( m23 + m32 ) / s;
		} else {
			let s = 2.0f32 * (1.0f32 + m33 - m11 - m22);

			self.w = ( m21 - m12 ) / s;
			self.x = ( m13 + m31 ) / s;
			self.y = ( m23 + m32 ) / s;
			self.z = 0.25f32 * s;
		}
	}

	pub fn set_from_unit_vectors(&mut self, v_from: &Vector3, v_to: &Vector3) {
		let mut v1 = Vector3::new();
		let mut r = v_from.dot(v_to) + 1.0f32;

		if r < ::std::f32::EPSILON {
			r = 0.0f32;
			if v_from.get_x().abs() > v_from.get_z().abs() {
				v1.set(- v_from.get_y(), v_from.get_x(), 0.0f32);
			} else {
				v1.set(0.0f32, - v_from.get_z(), v_from.get_y());
			}
		} else {
			v1.cross_vectors(v_from, v_to);
		}

		self.x = v1.get_x();
		self.y = v1.get_y();
		self.z = v1.get_z();
		self.w = r;
		self.normalize();
	}

	pub fn inverse(&mut self) {
		self.conjugate();
		self.normalize();
	}

	pub fn conjugate(&mut self) {
		self.x *= - 1.0f32;
		self.y *= - 1.0f32;
		self.z *= - 1.0f32;
	}

	pub fn dot(&self, v: &Quaternion) -> f32 {
		self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
	}

	pub fn length_sq(&self) -> f32 {
		self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
	}

	pub fn length(&self) -> f32 {
		self.length_sq().sqrt()
	}

	pub fn normalize(&mut self) {
		let mut l = self.length();
		if l == 0.0f32 {
			self.x = 0.0f32;
			self.y = 0.0f32;
			self.z = 0.0f32;
			self.w = 1.0f32;
		} else {
			l = 1.0f32 / l;
			self.x = self.x * l;
			self.y = self.y * l;
			self.z = self.z * l;
			self.w = self.w * l;
		}
	}

	pub fn multiply(&mut self, q: &Quaternion) {
		let s = &self.clone();
		self.multiply_quaternions(s, q);
	}

	pub fn premultiply(&mut self, q: &Quaternion) {
		let s = &self.clone();
		self.multiply_quaternions(q, s);
	}

	pub fn multiply_quaternions(&mut self, a: &Quaternion, b: &Quaternion) {
		let qax = a.x;
		let qay = a.y;
		let qaz = a.z;
		let qaw = a.w;
		let qbx = b.x;
		let qby = b.y;
		let qbz = b.z;
		let qbw = b.w;

		self.x = qax * qbw + qaw * qbx + qay * qbz - qaz * qby;
		self.y = qay * qbw + qaw * qby + qaz * qbx - qax * qbz;
		self.z = qaz * qbw + qaw * qbz + qax * qby - qay * qbx;
		self.w = qaw * qbw - qax * qbx - qay * qby - qaz * qbz;
	}
}