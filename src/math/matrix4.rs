use super::quaternion::Quaternion;

#[derive(Debug)]
pub struct Matrix4 {
	elements: [f32; 16] 
}

impl Matrix4 {
	pub fn new() -> Matrix4 {
		unimplemented!();
	}

	pub fn get_elements(&self) -> &[f32; 16] {
		&self.elements
	}

	pub fn make_rotation_from_quaternion(&mut self, q: &Quaternion) {
		unimplemented!();
	}
}