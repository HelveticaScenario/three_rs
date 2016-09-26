use super::super::math::matrix4::Matrix4;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
	pub matrix_world_inverse: Matrix4,
	pub projection_matrix: Matrix4,
}

impl Camera {
	pub fn new() -> Camera {
		Camera {
			matrix_world_inverse: Matrix4::new(),
			projection_matrix: Matrix4::new(),
		}
	}

	pub fn get_matrix_world_inverse(&self) -> &Matrix4 {
		&self.matrix_world_inverse
	}

	pub fn get_projection_matrix(&self) -> &Matrix4 {
		&self.projection_matrix
	}
}