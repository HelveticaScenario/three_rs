use super::matrix4::Matrix4;
use super::vector3::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Matrix3 {
	elements: [f32; 9] 
}

impl Matrix3 {
	pub fn new() -> Matrix3 {
		let elements = [
			1.0, 0.0, 0.0,
			0.0, 1.0, 0.0,
			0.0, 0.0, 1.0,
		];
		Matrix3 {
			elements: elements
		}
	}

	pub fn get_elements(&self) -> &[f32; 9] {
		&self.elements
	}

	pub fn set(&mut self, n11: f32, n12: f32, n13: f32, n21: f32, n22: f32, n23: f32, n31: f32, n32: f32, n33: f32) {
		self.elements[ 0 ] = n11;
		self.elements[ 1 ] = n21;
		self.elements[ 2 ] = n31;

		self.elements[ 3 ] = n12;
		self.elements[ 4 ] = n22;
		self.elements[ 5 ] = n32;

		self.elements[ 6 ] = n13;
		self.elements[ 7 ] = n23;
		self.elements[ 8 ] = n33;
	}

	pub fn identity(&mut self) {
		self.set(
			1.0, 0.0, 0.0,
			0.0, 1.0, 0.0,
			0.0, 0.0, 1.0
		);
	}

	pub fn copy(&mut self, m: &Matrix3) {
		self.set(
			m.elements[ 0 ], m.elements[ 3 ], m.elements[ 6 ],
			m.elements[ 1 ], m.elements[ 4 ], m.elements[ 7 ],
			m.elements[ 2 ], m.elements[ 5 ], m.elements[ 8 ]
		);
	}

	pub fn set_from_matrix4(&mut self, m: &Matrix4) {
		let me = m.get_elements();
		self.set(
			me[ 0 ], me[ 4 ], me[  8 ],
			me[ 1 ], me[ 5 ], me[  9 ],
			me[ 2 ], me[ 6 ], me[ 10 ]
		);
	}

	pub fn apply_to_vector3_array(&self, array: &mut [f32], offset: Option<usize>, length: Option<usize>) {
		let mut v1 = Vector3::new();
		let offset: usize = match offset {
			Some(off) => off,
			None => 0,
		};
		let length: usize = match length {
			Some(len) => len,
			None => array.len(),
		};
		for i in 0..length {
			if i % 3 != 0 {
				continue;
			}
			v1.copy_from_array( array, Some(i + offset));
			v1.apply_matrix3( self );
			v1.copy_to_array( array, Some(i + offset));
		}
	}

	pub fn multiply_scalar(&mut self, s: f32) {
		self.elements[ 0 ] *= s;
		self.elements[ 3 ] *= s;
		self.elements[ 6 ] *= s;

		self.elements[ 1 ] *= s;
		self.elements[ 4 ] *= s;
		self.elements[ 7 ] *= s;

		self.elements[ 2 ] *= s;
		self.elements[ 5 ] *= s;
		self.elements[ 8 ] *= s;
	}

	pub fn determinant(&self) -> f32 {
		let a = self.elements[ 0 ];
		let b = self.elements[ 1 ];
		let c = self.elements[ 2 ];
		let d = self.elements[ 3 ];
		let e = self.elements[ 4 ];
		let f = self.elements[ 5 ];
		let g = self.elements[ 6 ];
		let h = self.elements[ 7 ];
		let i = self.elements[ 8 ];

		a * e * i - a * f * h - b * d * i + b * f * g + c * d * h - c * e * g
	}

	pub fn get_inverse(&mut self, matrix: &Matrix3, throw_on_degenerate: bool) {
		let me = matrix.elements;
		let n11 = me[ 0 ];
		let n21 = me[ 1 ];
		let n31 = me[ 2 ];

		let n12 = me[ 3 ];
		let n22 = me[ 4 ];
		let n32 = me[ 5 ];

		let n13 = me[ 6 ];
		let n23 = me[ 7 ];
		let n33 = me[ 8 ];

		let t11 = n33 * n22 - n32 * n23;
		let t12 = n32 * n13 - n33 * n12;
		let t13 = n23 * n12 - n22 * n13;

		let det = n11 * t11 + n21 * t12 + n31 * t13;

		if det == 0.0 {
			let msg = "Matrix3.getInverse(): can't invert matrix, determinant is 0";

			if throw_on_degenerate {
				panic!("{}", msg);
			} else {
				println!("{}", msg);
			}

			return self.identity();
		}

		let det_inv = 1.0 / det;

		self.elements[ 0 ] = t11 * det_inv;
		self.elements[ 1 ] = ( n31 * n23 - n33 * n21 ) * det_inv;
		self.elements[ 2 ] = ( n32 * n21 - n31 * n22 ) * det_inv;

		self.elements[ 3 ] = t12 * det_inv;
		self.elements[ 4 ] = ( n33 * n11 - n31 * n13 ) * det_inv;
		self.elements[ 5 ] = ( n31 * n12 - n32 * n11 ) * det_inv;

		self.elements[ 6 ] = t13 * det_inv;
		self.elements[ 7 ] = ( n21 * n13 - n23 * n11 ) * det_inv;
		self.elements[ 8 ] = ( n22 * n11 - n21 * n12 ) * det_inv;
	}

	pub fn transpose(&mut self) {
		let mut tmp: f32;
		tmp = self.elements[ 1 ];
		self.elements[ 1 ] = self.elements[ 3 ];
		self.elements[ 3 ] = tmp;

		tmp = self.elements[ 2 ];
		self.elements[ 2 ] = self.elements[ 6 ];
		self.elements[ 6 ] = tmp;

		tmp = self.elements[ 5 ];
		self.elements[ 5 ] = self.elements[ 7 ];
		self.elements[ 7 ] = tmp;
	}

	pub fn get_normal_matrix(&mut self, matrix4: &Matrix4) {
		self.set_from_matrix4(matrix4);
		let s = *self;
		self.get_inverse(&s, false);
		self.transpose();
	}

	pub fn transpose_into_array(&self, r: &mut [f32]) {
		r[ 0 ] = self.elements[ 0 ];
		r[ 1 ] = self.elements[ 3 ];
		r[ 2 ] = self.elements[ 6 ];
		r[ 3 ] = self.elements[ 1 ];
		r[ 4 ] = self.elements[ 4 ];
		r[ 5 ] = self.elements[ 7 ];
		r[ 6 ] = self.elements[ 2 ];
		r[ 7 ] = self.elements[ 5 ];
		r[ 8 ] = self.elements[ 8 ];
	}

	pub fn copy_from_array(&mut self, array: &[f32]) {
		self.elements.copy_from_slice(array);
	}

	pub fn copy_to_array(&self, array: &mut [f32], offset: Option<usize>) {
		let offset: usize = match offset {
			Some(off) => off,
			None => 0,
		};

		array[ offset ] = self.elements[ 0 ];
		array[ offset + 1 ] = self.elements[ 1 ];
		array[ offset + 2 ] = self.elements[ 2 ];

		array[ offset + 3 ] = self.elements[ 3 ];
		array[ offset + 4 ] = self.elements[ 4 ];
		array[ offset + 5 ] = self.elements[ 5 ];

		array[ offset + 6 ] = self.elements[ 6 ];
		array[ offset + 7 ] = self.elements[ 7 ];
		array[ offset + 8 ]  = self.elements[ 8 ];
	}
}