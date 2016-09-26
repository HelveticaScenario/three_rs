use super::matrix4::Matrix4;
use super::vector3::Vector3;
use super::quaternion::Quaternion;
use super::math_static::clamp;
use std::mem;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RotationOrders {
	XYZ,
	YZX,
	ZXY,
	XZY,
	YXZ,
	ZYX,
}

impl From<u8> for RotationOrders {
    fn from(t:u8) -> RotationOrders {
        assert!(RotationOrders::XYZ as u8 <= t && t <= RotationOrders::ZYX as u8);
        unsafe { mem::transmute(t) }
    }
}

pub static mut DEFAULT_ORDER: RotationOrders = RotationOrders::XYZ;

#[derive(Debug, Clone, Copy)]
pub struct Euler {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub order: RotationOrders,
}

impl Euler {
	pub fn new() -> Euler {
		Euler {
			x: 0.0f32,
			y: 0.0f32,
			z: 0.0f32,
			order: unsafe {DEFAULT_ORDER},
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

	pub fn set_order(&mut self, order: RotationOrders) {
		self.order = order;
	}

	pub fn get_order(&self) -> RotationOrders {
		self.order
	}

	pub fn set(&mut self, x: f32, y: f32, z: f32, order: RotationOrders) {
		self.x = x;
		self.y = y;
		self.z = z;
		self.order = order;
	}

	pub fn copy(&mut self, euler: &Euler) {
		self.x = euler.x;
		self.y = euler.y;
		self.z = euler.z;
		self.order = euler.order;
	}

	pub fn set_from_rotation_matrix(&mut self, m: &Matrix4, order: Option<RotationOrders>) {
		let te = m.get_elements();
		let m11 = te[ 0 ];
		let m12 = te[ 4 ];
		let m13 = te[ 8 ];
		let m21 = te[ 1 ];
		let m22 = te[ 5 ];
		let m23 = te[ 9 ];
		let m31 = te[ 2 ];
		let m32 = te[ 6 ];
		let m33 = te[ 10 ];
		let order = match order {
			Some(ord) => ord,
			None => self.order,
		};

		match order {
			RotationOrders::XYZ => {
				self.y = clamp(m13, -1.0, 1.0).asin();

				if m13.abs() < 0.99999f32 {
					self.x = (-m23).atan2(m33);
					self.z = (-m12).atan2(m11);
				} else {
					self.x = m32.atan2(m22);
					self.z = 0.0f32;
				}
			},
			RotationOrders::YXZ => {
				self.x = (-clamp(m23, -1.0, 1.0)).asin();

				if m23.abs() < 0.99999f32 {
					self.y = m13.atan2(m33);
					self.z = m21.atan2(m22);
				} else {
					self.y = (-m31).atan2(m11);
					self.z = 0.0f32;
				}
			},
			RotationOrders::ZXY => {
				self.x = clamp(m32, -1.0, 1.0).asin();

				if m32.abs() < 0.99999f32 {
					self.y = (-m31).atan2(m33);
					self.z = (-m12).atan2(m22);
				} else {
					self.y = 0.0f32;
					self.z = m21.atan2(m11);
				}
			},
			RotationOrders::ZYX => {
				self.y = (-clamp(m31, -1.0, 1.0)).asin();

				if m31.abs() < 0.99999f32 {
					self.x = m32.atan2(m33);
					self.z = m21.atan2(m11);
				} else {
					self.x = 0.0f32;
					self.z = (-m12).atan2(m22);
				}
			},
			RotationOrders::YZX => {
				self.z = clamp(m21, -1.0, 1.0).asin();

				if m21.abs() < 0.99999f32 {
					self.x = (-m23).atan2(m22);
					self.y = (-m31).atan2(m11);
				} else {
					self.x = 0.0f32;
					self.y = m13.atan2(m33);
				}
			},
			RotationOrders::XZY => {
				self.z = (-clamp(m21, -1.0, 1.0)).asin();

				if m12.abs() < 0.99999f32 {
					self.x = m32.atan2(m22);
					self.y = m13.atan2(m11);
				} else {
					self.x = (-m23).atan2(m33);
					self.y = 0.0f32;
				}
			},
		}

		self.order = order;
	}

	pub fn set_from_quaternion(&mut self, q: &Quaternion, order: Option<RotationOrders>) {
		let mut matrix = Matrix4::new();
		matrix.make_rotation_from_quaternion(q);
		self.set_from_rotation_matrix(&matrix, order);
	}

	pub fn set_from_vector3(&mut self, v: &Vector3, order: Option<RotationOrders>) {
		let order = match order {
			Some(ord) => ord,
			None => self.order,
		};
		self.set(v.get_x(), v.get_y(), v.get_z(), order);
	}

	pub fn reorder(&mut self, new_order: RotationOrders) {
		let mut q = Quaternion::new();
		q.set_from_euler(self);
		self.set_from_quaternion(&q, Some(new_order));
	}

	pub fn equals(&mut self, euler: &Euler) -> bool {
		( euler.x == self.x ) && ( euler.y == self.y ) && ( euler.z == self.z ) && ( euler.order == self.order )
	}

	pub fn copy_from_array(&mut self, array: &[f32]) {
		self.x = array[0];
		self.y = array[1];
		self.z = array[2];
		// I know this is bad, but its necessary to get api compatibility
		if array.len() >= 4 {
			self.order = RotationOrders::from(array[3] as u8);
		}
	}

	pub fn copy_to_array(&self, array: &mut [f32], offset: Option<usize>) {
		let offset = match offset {
			Some(off) => off,
			None => 0usize,
		};
		array[ offset ] = self.x;
		array[ offset + 1 ] = self.y;
		array[ offset + 2 ] = self.z;
		// I know this is bad, but its necessary to get api compatibility
		array[ offset + 3 ] = (self.order as u8) as f32;
	}

	pub fn to_vector3(&self, vector: &mut Vector3) {
		vector.set(self.x, self.y, self.z);
	}
}