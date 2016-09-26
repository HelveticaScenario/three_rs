use super::vector3::Vector3;
use super::math_static::clamp;

#[derive(Debug, Clone, Copy)]
pub struct Spherical {
	pub radius: f32,
	pub phi: f32,
	pub theta: f32,
}

impl Spherical {
	pub fn new() -> Spherical {
		Spherical {
			radius: 1.0,
			phi: 0.0,
			theta: 0.0,
		}
	}

	pub fn get_radius(&self) -> f32 {
		self.radius
	}

	pub fn get_phi(&self) -> f32 {
		self.phi
	}

	pub fn get_theta(&self) -> f32 {
		self.theta
	}

	pub fn set_radius(&mut self, v: f32) {
		self.radius = v;
	}

	pub fn set_phi(&mut self, v: f32) {
		self.phi = v;
	}

	pub fn set_theta(&mut self, v: f32) {
		self.theta = v;
	}

	pub fn set(&mut self, radius: f32, phi: f32, theta: f32) {
		self.radius = radius;
		self.phi = phi;
		self.theta = theta;
	}

	pub fn copy(&mut self, other: &Spherical) {
		self.radius = other.radius;
		self.phi = other.phi;
		self.theta = other.theta;
	}

	pub fn make_safe(&mut self) {
		let eps = 0.000001f32;
		self.phi = eps.max((::std::f32::consts::PI - eps).min(self.phi));
	}

	pub fn set_from_vector3(&mut self, vec3: Vector3) {
		self.radius = vec3.length();

		if self.radius == 0.0 {
			self.theta = 0.0;
			self.phi = 0.0;
		} else {
			self.theta = vec3.get_x().atan2( vec3.get_z() ); // equator angle around y-up axis
			self.phi = clamp( vec3.get_y() / self.radius, - 1.0, 1.0 ).acos(); // polar angle
		}
	}
}