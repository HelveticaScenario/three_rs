#[derive(Debug)]
pub struct Matrix3 {
	elements: [f32; 9] 
}

impl Matrix3 {
	pub fn new() -> Matrix3 {
		unimplemented!();
	}

	pub fn get_elements(&self) -> &[f32; 9] {
		&self.elements
	}
}