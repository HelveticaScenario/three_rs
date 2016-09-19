#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
	x: f32,
	y: f32,
}

impl Vector2 {
	pub fn new() -> Vector2 {
		Vector2 {
			x: 0.0,
			y: 0.0,
		}
	}

	pub fn get_width(&self) -> f32 {
		self.x
	}

	pub fn set_width(&mut self, value: f32) {
		self.x = value;
	}

	pub fn get_height(&self) -> f32 {
		self.y
	}

	pub fn set_height(&mut self, value: f32) {
		self.y = value;
	}

	pub fn set(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
	}

	pub fn set_scalar(&mut self, scalar: f32) {
		self.x = scalar;
		self.y = scalar;
	}

	pub fn set_x(&mut self, x: f32) {
		self.x = x;
	}

	pub fn set_y(&mut self, y: f32) {
		self.y = y;
	}

	pub fn set_component(&mut self, index: i32, value: f32) {
		match index {
			0 => self.x = value,
			1 => self.y = value,
			_ => panic!("index out of range: {:?}", index)
		};
	}

	pub fn get_component(&mut self, index: i32) -> f32 {
		match index {
			0 => self.x,
			1 => self.y,
			_ => panic!("index out of range: {:?}", index)
		}
	}

	pub fn add(&mut self, v: &Vector2) {
		self.x += v.x;
		self.y += v.y;
	}

	pub fn add_scalar(&mut self, s: f32) {
		self.x += s;
		self.y += s;
	}

	pub fn add_vectors(&mut self, a: &Vector2, b: &Vector2) {
		self.x = a.x + b.x;
		self.y = a.y + b.y;
	}

	pub fn add_scaled_vector(&mut self, v: &Vector2, s: f32) {
		self.x += v.x * s;
		self.y += v.y * s;
	}

	pub fn sub(&mut self, v: &Vector2) {
		self.x -= v.x;
		self.y -= v.y;
	}

	pub fn sub_scalar(&mut self, s: f32) {
		self.x -= s;
		self.y -= s;
	}

	pub fn sub_vectors(&mut self, a: &Vector2, b: &Vector2) {
		self.x = a.x - b.x;
		self.y = a.y - b.y;
	}

	pub fn multiply(&mut self, v: &Vector2) {
		self.x *= v.x;
		self.y *= v.y;
	}

	pub fn multiply_scalar(&mut self, scalar: f32) {
		if scalar.is_finite() {
			self.x *= scalar;
			self.y *= scalar;
		} else {
			self.x = 0.0;
			self.y = 0.0;
		}
	}

	pub fn divide(&mut self, v: &Vector2) {
		self.x /= v.x;
		self.y /= v.y;
	}

	pub fn divide_scalar(&mut self, scalar: f32) {
		self.multiply_scalar(1.0 / scalar);
	}

	pub fn min(&mut self, v: &Vector2) {
		self.x = self.x.min(v.x);
		self.y = self.y.min(v.y);
	}

	pub fn max(&mut self, v: &Vector2) {
		self.x = self.x.max(v.x);
		self.y = self.y.max(v.y);
	}

	pub fn clamp(&mut self, min: &Vector2, max: &Vector2) {
		self.x = min.x.max(max.x.min(self.x));
		self.y = min.y.max(max.y.min(self.y));
	}

	pub fn clamp_scalar(&mut self, min_val: f32, max_val: f32) {
		self.clamp(&Vector2 {
			x: min_val,
			y: min_val,
		}, &Vector2 {
			x: max_val,
			y: max_val,
		});
	}

	pub fn clamp_length(&mut self, min: f32, max: f32) {
		let length = self.length();

		self.multiply_scalar(min.max(max.min(length)) / length);
	}

	pub fn floor(&mut self) {
		self.x = self.x.floor();
		self.y = self.y.floor();
	}

	pub fn ceil(&mut self) {
		self.x = self.x.ceil();
		self.y = self.y.ceil();
	}

	pub fn round(&mut self) {
		self.x = self.x.round();
		self.y = self.y.round();
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
	}

	pub fn negate(&mut self) {
		self.x = -self.x;
		self.y = -self.y;
	}

	pub fn dot(&self, v: &Vector2) -> f32 {
		(self.x * v.x) + (self.y * v.y)
	}

	pub fn length_sq(&self) -> f32 {
		(self.x * self.x) + (self.y * self.y)
	}

	pub fn length(&self) -> f32 {
		self.length_sq().sqrt()
	}

	pub fn length_manhattan(&self) -> f32 {
		self.x.abs() + self.y.abs()
	}

	pub fn normalize(&mut self) {
		let length = {
			self.length()
		};
		self.divide_scalar(length)
	}

	pub fn angle(&self) -> f32 {
		let mut angle = self.y.atan2(self.x);
		if angle < 0.0 {
			angle += 2.0 * ::std::f32::consts::PI;
		}
		angle
	}

	pub fn distance_to(&self, v: &Vector2) -> f32 {
		self.distance_to_squared(v).sqrt()
	}

	pub fn distance_to_squared(&self, v: &Vector2) -> f32 {
		let dx = self.x - v.x;
		let dy = self.y - v.y;
		(dx * dx) + (dy * dy)
	}

	pub fn distance_to_manhattan(&self, v: &Vector2) -> f32 {
		((self.x - v.x).abs()) + ((self.y - v.y).abs())
	}

	pub fn set_length(&mut self, length: f32) {
		let l = {
			length / self.length()
		};
		self.multiply_scalar(l);
	}

	pub fn lerp(&mut self, v: &Vector2, alpha: f32) {
		self.x += (v.x - self.x) * alpha;
		self.y += (v.y - self.y) * alpha;
	}

	pub fn lerp_vectors(&mut self, v1: &Vector2, v2: &Vector2, alpha: f32) {
		self.sub_vectors(v2, v1);
		self.multiply_scalar(alpha);
		self.add(v1);
	}

	pub fn equals(&self, v: &Vector2) -> bool {
		(v.x == self.x) && (v.y == self.y)
	}

	pub fn copy_from_array(&mut self, array: &[f32]) {
		self.copy_from_array_offset(array, 0);
	}

	pub fn copy_from_array_offset(&mut self, array: &[f32], offset: usize) {
		self.x = array[offset];
		self.y = array[offset + 1];
	}

	pub fn copy_to_array(&self, array: &mut [f32]) {
		self.copy_to_array_offset(array, 0);
	}

	pub fn copy_to_array_offset(&self, array: &mut [f32], offset: usize) {
		array[offset] = self.x;
		array[offset + 1] = self.y;
	}

	pub fn rotate_around(&mut self, center: &Vector2, angle: f32) {
		let c = angle.cos();
		let s = angle.sin();

		let x = self.x - center.x;
		let y = self.y - center.y;

		self.x = (x * c) - (y * s) + center.x;
		self.y = (x * s) + (y * c) + center.y;
	}

	pub fn copy(&mut self, v: &Vector2) {
		self.x = v.x;
		self.y = v.y;
	}

}
