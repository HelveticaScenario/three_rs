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


}