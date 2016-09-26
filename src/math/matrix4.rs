use super::quaternion::Quaternion;
use super::vector3::Vector3;
use super::euler::{Euler, RotationOrders};

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
	pub elements: [f32; 16] 
}

impl Matrix4 {
	pub fn new() -> Matrix4 {
		let elements = [
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0
		];
		Matrix4 {
			elements: elements
		}
	}

	pub fn get_elements(&self) -> &[f32; 16] {
		&self.elements
	}

	pub fn set(&mut self, n11: f32, n12: f32, n13: f32, n14: f32, n21: f32, n22: f32, n23: f32, n24: f32, n31: f32, n32: f32, n33: f32, n34: f32, n41: f32, n42: f32, n43: f32, n44: f32) {
		self.elements[ 0 ] = n11;
		self.elements[ 4 ] = n12;
		self.elements[ 8 ] = n13;
		self.elements[ 12] = n14;
		self.elements[ 1 ] = n21;
		self.elements[ 5 ] = n22;
		self.elements[ 9 ] = n23;
		self.elements[ 13] = n24;
		self.elements[ 2 ] = n31;
		self.elements[ 6 ] = n32;
		self.elements[ 10] = n33;
		self.elements[ 14] = n34;
		self.elements[ 3 ] = n41;
		self.elements[ 7 ] = n42;
		self.elements[ 11] = n43;
		self.elements[ 15] = n44;
	}

	pub fn identity(&mut self) {
		self.set(
			1.0f32, 0.0f32, 0.0f32, 0.0f32,
			0.0f32, 1.0f32, 0.0f32, 0.0f32,
			0.0f32, 0.0f32, 1.0f32, 0.0f32,
			0.0f32, 0.0f32, 0.0f32, 1.0f32
		);
	}

	pub fn copy(&mut self, m: &Matrix4) {
		self.elements.clone_from_slice(&m.elements);
	}

	pub fn copy_position(&mut self, m: &Matrix4) {
		self.elements[ 12 ] = self.elements[ 12 ];
		self.elements[ 13 ] = self.elements[ 13 ];
		self.elements[ 14 ] = self.elements[ 14 ];
	}

	pub fn extract_basis(&self, x_axis: &mut Vector3, y_axis: &mut Vector3, z_axis: &mut Vector3) {
		x_axis.set_from_matrix_column(self, 0);
		y_axis.set_from_matrix_column(self, 1);
		z_axis.set_from_matrix_column(self, 2);
	}

	pub fn make_basis(&mut self, x_axis: &Vector3, y_axis: &Vector3, z_axis: &Vector3) {
		self.set(
			x_axis.get_x(), y_axis.get_x(), z_axis.get_x(), 0.0,
			x_axis.get_y(), y_axis.get_y(), z_axis.get_y(), 0.0,
			x_axis.get_z(), y_axis.get_z(), z_axis.get_z(), 0.0,
			0.0,       0.0,       0.0,       1.0
		);
	}

	pub fn extract_rotation(&mut self, m: &Matrix4) {
		let mut v1 = Vector3::new();
		v1.set_from_matrix_column( m, 0 );
		let scale_x = 1.0 / v1.length();
		v1.set_from_matrix_column( m, 1 );
		let scale_y = 1.0 / v1.length();
		v1.set_from_matrix_column( m, 2 );
		let scale_z = 1.0 / v1.length();

		self.elements[ 0 ] = m.elements[ 0 ] * scale_x;
		self.elements[ 1 ] = m.elements[ 1 ] * scale_x;
		self.elements[ 2 ] = m.elements[ 2 ] * scale_x;

		self.elements[ 4 ] = m.elements[ 4 ] * scale_y;
		self.elements[ 5 ] = m.elements[ 5 ] * scale_y;
		self.elements[ 6 ] = m.elements[ 6 ] * scale_y;

		self.elements[ 8 ] = m.elements[ 8 ] * scale_z;
		self.elements[ 9 ] = m.elements[ 9 ] * scale_z;
		self.elements[ 10] = m.elements[ 10] * scale_z;
	}

	pub fn make_rotation_from_euler(&mut self, euler: &Euler) {
		let x = euler.get_x();
		let y = euler.get_y();
		let z = euler.get_z();
		let a = x.cos();
		let b = x.sin();
		let c = y.cos();
		let d = y.sin();
		let e = z.cos();
		let f = z.sin();

		match euler.get_order() {
			RotationOrders::XYZ => {

				let ae = a * e;
				let af = a * f;
				let be = b * e; 
				let bf = b * f;

				self.elements[ 0 ] = c * e;
				self.elements[ 4 ] = - c * f;
				self.elements[ 8 ] = d;

				self.elements[ 1 ] = af + be * d;
				self.elements[ 5 ] = ae - bf * d;
				self.elements[ 9 ] = - b * c;

				self.elements[ 2 ] = bf - ae * d;
				self.elements[ 6 ] = be + af * d;
				self.elements[ 10 ] = a * c;

			},
			RotationOrders::YXZ => {

				let ce = c * e;
				let cf = c * f;
				let de = d * e; 
				let df = d * f;

				self.elements[ 0 ] = ce + df * b;
				self.elements[ 4 ] = de * b - cf;
				self.elements[ 8 ] = a * d;

				self.elements[ 1 ] = a * f;
				self.elements[ 5 ] = a * e;
				self.elements[ 9 ] = - b;

				self.elements[ 2 ] = cf * b - de;
				self.elements[ 6 ] = df + ce * b;
				self.elements[ 10 ] = a * c;

			},
			RotationOrders::ZXY => {

				let ce = c * e;
				let cf = c * f;
				let de = d * e; 
				let df = d * f;

				self.elements[ 0 ] = ce - df * b;
				self.elements[ 4 ] = - a * f;
				self.elements[ 8 ] = de + cf * b;

				self.elements[ 1 ] = cf + de * b;
				self.elements[ 5 ] = a * e;
				self.elements[ 9 ] = df - ce * b;

				self.elements[ 2 ] = - a * d;
				self.elements[ 6 ] = b;
				self.elements[ 10 ] = a * c;

			},
			RotationOrders::ZYX => {

				let ae = a * e;
				let af = a * f;
				let be = b * e; 
				let bf = b * f;

				self.elements[ 0 ] = c * e;
				self.elements[ 4 ] = be * d - af;
				self.elements[ 8 ] = ae * d + bf;

				self.elements[ 1 ] = c * f;
				self.elements[ 5 ] = bf * d + ae;
				self.elements[ 9 ] = af * d - be;

				self.elements[ 2 ] = - d;
				self.elements[ 6 ] = b * c;
				self.elements[ 10 ] = a * c;

			},
			RotationOrders::YZX => {

				let ac = a * c;
				let ad = a * d;
				let bc = b * c; 
				let bd = b * d;

				self.elements[ 0 ] = c * e;
				self.elements[ 4 ] = bd - ac * f;
				self.elements[ 8 ] = bc * f + ad;

				self.elements[ 1 ] = f;
				self.elements[ 5 ] = a * e;
				self.elements[ 9 ] = - b * e;

				self.elements[ 2 ] = - d * e;
				self.elements[ 6 ] = ad * f + bc;
				self.elements[ 10 ] = ac - bd * f;

			},
			RotationOrders::XZY => {

				let ac = a * c;
				let ad = a * d;
				let bc = b * c; 
				let bd = b * d;

				self.elements[ 0 ] = c * e;
				self.elements[ 4 ] = - f;
				self.elements[ 8 ] = d * e;

				self.elements[ 1 ] = ac * f + bd;
				self.elements[ 5 ] = a * e;
				self.elements[ 9 ] = ad * f - bc;

				self.elements[ 2 ] = bc * f - ad;
				self.elements[ 6 ] = b * e;
				self.elements[ 10 ] = bd * f + ac;

			}
		}
		

		// last column
		self.elements[ 3 ] = 0.0;
		self.elements[ 7 ] = 0.0;
		self.elements[ 11 ] = 0.0;

		// bottom row
		self.elements[ 12 ] = 0.0;
		self.elements[ 13 ] = 0.0;
		self.elements[ 14 ] = 0.0;
		self.elements[ 15 ] = 1.0;
	}

	pub fn make_rotation_from_quaternion(&mut self, q: &Quaternion) {
		let x = q.get_x();
		let y = q.get_y();
		let z = q.get_z();
		let w = q.get_w();
		let x2 = x + x;
		let y2 = y + y;
		let z2 = z + z;
		let xx = x * x2;
		let xy = x * y2;
		let xz = x * z2;
		let yy = y * y2;
		let yz = y * z2;
		let zz = z * z2;
		let wx = w * x2;
		let wy = w * y2;
		let wz = w * z2;

		self.elements[ 0 ] = 1.0 - ( yy + zz );
		self.elements[ 4 ] = xy - wz;
		self.elements[ 8 ] = xz + wy;

		self.elements[ 1 ] = xy + wz;
		self.elements[ 5 ] = 1.0 - ( xx + zz );
		self.elements[ 9 ] = yz - wx;

		self.elements[ 2 ] = xz - wy;
		self.elements[ 6 ] = yz + wx;
		self.elements[ 10 ] = 1.0 - ( xx + yy );

		// last column
		self.elements[ 3 ] = 0.0;
		self.elements[ 7 ] = 0.0;
		self.elements[ 11 ] = 0.0;

		// bottom row
		self.elements[ 12 ] = 0.0;
		self.elements[ 13 ] = 0.0;
		self.elements[ 14 ] = 0.0;
		self.elements[ 15 ] = 1.0;
	}

	pub fn look_at(&mut self, eye: &Vector3, target: &Vector3, up: &Vector3) {

		let mut x = Vector3::new();
		let mut y = Vector3::new();
		let mut z = Vector3::new();

		z.sub_vectors( eye, target );
		z.normalize();

		if z.length_sq() == 0.0 {
			z.set_z(1.0);
		}

		x.cross_vectors( up, &z );
		x.normalize();

		if x.length_sq() == 0.0 {
			let inc = {
				z.get_z() + 0.0001
			};
			z.set_z(inc);
			x.cross_vectors( up, &z );
			x.normalize();
		}

		y.cross_vectors( &z, &x );


		self.elements[ 0 ] = x.get_x();
		self.elements[ 4 ] = y.get_x();
		self.elements[ 8 ] = z.get_x();
		self.elements[ 1 ] = x.get_y();
		self.elements[ 5 ] = y.get_y();
		self.elements[ 9 ] = z.get_y();
		self.elements[ 2 ] = x.get_z();
		self.elements[ 6 ] = y.get_z();
		self.elements[ 10 ] = z.get_z();		
	}

	pub fn multiply(&mut self, m: &Matrix4) {
		let s = *self;
		self.multiply_matrices(&s, m);
	}
	
	pub fn premultiply(&mut self, m: &Matrix4) {
		let s = *self;
		self.multiply_matrices(m, &s);
	}

	pub fn multiply_matrices(&mut self, a: &Matrix4, b: &Matrix4) {
		let a11 = a.elements[ 0 ];
		let a12 = a.elements[ 4 ];
		let a13 = a.elements[ 8 ];
		let a14 = a.elements[ 12 ];
		let a21 = a.elements[ 1 ];
		let a22 = a.elements[ 5 ];
		let a23 = a.elements[ 9 ];
		let a24 = a.elements[ 13 ];
		let a31 = a.elements[ 2 ];
		let a32 = a.elements[ 6 ];
		let a33 = a.elements[ 10 ];
		let a34 = a.elements[ 14 ];
		let a41 = a.elements[ 3 ];
		let a42 = a.elements[ 7 ];
		let a43 = a.elements[ 11 ];
		let a44 = a.elements[ 15 ];

		let b11 = b.elements[ 0 ];
		let b12 = b.elements[ 4 ];
		let b13 = b.elements[ 8 ];
		let b14 = b.elements[ 12 ];
		let b21 = b.elements[ 1 ];
		let b22 = b.elements[ 5 ];
		let b23 = b.elements[ 9 ];
		let b24 = b.elements[ 13 ];
		let b31 = b.elements[ 2 ];
		let b32 = b.elements[ 6 ];
		let b33 = b.elements[ 10 ];
		let b34 = b.elements[ 14 ];
		let b41 = b.elements[ 3 ];
		let b42 = b.elements[ 7 ];
		let b43 = b.elements[ 11 ];
		let b44 = b.elements[ 15 ];

		self.elements[ 0 ] = a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41;
		self.elements[ 4 ] = a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42;
		self.elements[ 8 ] = a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43;
		self.elements[ 12 ] = a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44;

		self.elements[ 1 ] = a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41;
		self.elements[ 5 ] = a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42;
		self.elements[ 9 ] = a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43;
		self.elements[ 13 ] = a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44;

		self.elements[ 2 ] = a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41;
		self.elements[ 6 ] = a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42;
		self.elements[ 10 ] = a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43;
		self.elements[ 14 ] = a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44;

		self.elements[ 3 ] = a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41;
		self.elements[ 7 ] = a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42;
		self.elements[ 11 ] = a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43;
		self.elements[ 15 ] = a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44;
	}

	pub fn multiply_to_array(&mut self, a: &Matrix4, b: &Matrix4, r: &mut [f32]) {
		self.multiply_matrices( a, b );

		r[ 0 ] = self.elements[ 0 ];
		r[ 1 ] = self.elements[ 1 ];
		r[ 2 ] = self.elements[ 2 ];
		r[ 3 ] = self.elements[ 3 ];
		r[ 4 ] = self.elements[ 4 ];
		r[ 5 ] = self.elements[ 5 ];
		r[ 6 ] = self.elements[ 6 ];
		r[ 7 ] = self.elements[ 7 ];
		r[ 8 ]  = self.elements[ 8 ];
		r[ 9 ]  = self.elements[ 9 ];
		r[ 10 ] = self.elements[ 10 ];
		r[ 11 ] = self.elements[ 11 ];
		r[ 12 ] = self.elements[ 12 ];
		r[ 13 ] = self.elements[ 13 ];
		r[ 14 ] = self.elements[ 14 ];
		r[ 15 ] = self.elements[ 15 ];
	}

	pub fn multiply_scalar(&mut self, s: f32) {
		self.elements[ 0 ] *= s;
		self.elements[ 4 ] *= s;
		self.elements[ 8 ] *= s;
		self.elements[ 12 ] *= s;
		self.elements[ 1 ] *= s;
		self.elements[ 5 ] *= s;
		self.elements[ 9 ] *= s;
		self.elements[ 13 ] *= s;
		self.elements[ 2 ] *= s;
		self.elements[ 6 ] *= s;
		self.elements[ 10 ] *= s;
		self.elements[ 14 ] *= s;
		self.elements[ 3 ] *= s;
		self.elements[ 7 ] *= s;
		self.elements[ 11 ] *= s;
		self.elements[ 15 ] *= s;
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
			v1.apply_matrix4( self );
			v1.copy_to_array( array, Some(i + offset));
		}
	}

	pub fn determinant(&self) -> f32 {
		let n11 = self.elements[ 0 ];
		let n12 = self.elements[ 4 ];
		let n13 = self.elements[ 8 ];
		let n14 = self.elements[ 12 ];
		let n21 = self.elements[ 1 ];
		let n22 = self.elements[ 5 ];
		let n23 = self.elements[ 9 ];
		let n24 = self.elements[ 13 ];
		let n31 = self.elements[ 2 ];
		let n32 = self.elements[ 6 ];
		let n33 = self.elements[ 10 ];
		let n34 = self.elements[ 14 ];
		let n41 = self.elements[ 3 ];
		let n42 = self.elements[ 7 ];
		let n43 = self.elements[ 11 ];
		let n44 = self.elements[ 15 ];

		(
			n41 * (
				   n14 * n23 * n32
				 - n13 * n24 * n32
				 - n14 * n22 * n33
				 + n12 * n24 * n33
				 + n13 * n22 * n34
				 - n12 * n23 * n34
			) +
			n42 * (
				   n11 * n23 * n34
				 - n11 * n24 * n33
				 + n14 * n21 * n33
				 - n13 * n21 * n34
				 + n13 * n24 * n31
				 - n14 * n23 * n31
			) +
			n43 * (
				   n11 * n24 * n32
				 - n11 * n22 * n34
				 - n14 * n21 * n32
				 + n12 * n21 * n34
				 + n14 * n22 * n31
				 - n12 * n24 * n31
			) +
			n44 * (
				 - n13 * n22 * n31
				 - n11 * n23 * n32
				 + n11 * n22 * n33
				 + n13 * n21 * n32
				 - n12 * n21 * n33
				 + n12 * n23 * n31
			)
		)
	}

	pub fn transpose(&mut self) {
		let mut tmp: f32;

		tmp = self.elements[ 1 ];
		self.elements[ 1 ] = self.elements[ 4 ];
		self.elements[ 4 ] = tmp;
		tmp = self.elements[ 2 ];
		self.elements[ 2 ] = self.elements[ 8 ];
		self.elements[ 8 ] = tmp;
		tmp = self.elements[ 6 ];
		self.elements[ 6 ] = self.elements[ 9 ];
		self.elements[ 9 ] = tmp;
		
		tmp = self.elements[ 3 ];
		self.elements[ 3 ] = self.elements[ 12 ];
		self.elements[ 12 ] = tmp;
		tmp = self.elements[ 7 ];
		self.elements[ 7 ] = self.elements[ 13 ];
		self.elements[ 13 ] = tmp;
		tmp = self.elements[ 11 ];
		self.elements[ 11 ] = self.elements[ 14 ];
		self.elements[ 14 ] = tmp;
	}

	pub fn set_position(&mut self, v: &Vector3) {
		self.elements[12] = v.get_x();
		self.elements[13] = v.get_y();
		self.elements[14] = v.get_z();
	}
	
	pub fn get_inverse(&mut self, m: &Matrix4, throw_on_degenerate: bool) {
		let n11 = m.elements[ 0 ];
		let n21 = m.elements[ 1 ];
		let n31 = m.elements[ 2 ];
		let n41 = m.elements[ 3 ];
		let n12 = m.elements[ 4 ];
		let n22 = m.elements[ 5 ];
		let n32 = m.elements[ 6 ];
		let n42 = m.elements[ 7 ];
		let n13 = m.elements[ 8 ];
		let n23 = m.elements[ 9 ];
		let n33 = m.elements[ 10 ];
		let n43 = m.elements[ 11 ];
		let n14 = m.elements[ 12 ];
		let n24 = m.elements[ 13 ];
		let n34 = m.elements[ 14 ];
		let n44 = m.elements[ 15 ];
		

		let t11 = n23 * n34 * n42 - n24 * n33 * n42 + n24 * n32 * n43 - n22 * n34 * n43 - n23 * n32 * n44 + n22 * n33 * n44;
		let t12 = n14 * n33 * n42 - n13 * n34 * n42 - n14 * n32 * n43 + n12 * n34 * n43 + n13 * n32 * n44 - n12 * n33 * n44;
		let t13 = n13 * n24 * n42 - n14 * n23 * n42 + n14 * n22 * n43 - n12 * n24 * n43 - n13 * n22 * n44 + n12 * n23 * n44;
		let t14 = n14 * n23 * n32 - n13 * n24 * n32 - n14 * n22 * n33 + n12 * n24 * n33 + n13 * n22 * n34 - n12 * n23 * n34;

		let det = n11 * t11 + n21 * t12 + n31 * t13 + n41 * t14;

		if det == 0.0 {

			let msg = "Matrix4.getInverse(): can't invert matrix; determinant is 0";

			if throw_on_degenerate == true {

				panic!("{}", msg);

			} else {

				println!("{}", msg);

			}

			self.identity();
			return;
		}

		let det_inv = 1.0 / det;

		self.elements[ 0 ] = t11 * det_inv;
		self.elements[ 1 ] = ( n24 * n33 * n41 - n23 * n34 * n41 - n24 * n31 * n43 + n21 * n34 * n43 + n23 * n31 * n44 - n21 * n33 * n44 ) * det_inv;
		self.elements[ 2 ] = ( n22 * n34 * n41 - n24 * n32 * n41 + n24 * n31 * n42 - n21 * n34 * n42 - n22 * n31 * n44 + n21 * n32 * n44 ) * det_inv;
		self.elements[ 3 ] = ( n23 * n32 * n41 - n22 * n33 * n41 - n23 * n31 * n42 + n21 * n33 * n42 + n22 * n31 * n43 - n21 * n32 * n43 ) * det_inv;

		self.elements[ 4 ] = t12 * det_inv;
		self.elements[ 5 ] = ( n13 * n34 * n41 - n14 * n33 * n41 + n14 * n31 * n43 - n11 * n34 * n43 - n13 * n31 * n44 + n11 * n33 * n44 ) * det_inv;
		self.elements[ 6 ] = ( n14 * n32 * n41 - n12 * n34 * n41 - n14 * n31 * n42 + n11 * n34 * n42 + n12 * n31 * n44 - n11 * n32 * n44 ) * det_inv;
		self.elements[ 7 ] = ( n12 * n33 * n41 - n13 * n32 * n41 + n13 * n31 * n42 - n11 * n33 * n42 - n12 * n31 * n43 + n11 * n32 * n43 ) * det_inv;

		self.elements[ 8 ] = t13 * det_inv;
		self.elements[ 9 ] = ( n14 * n23 * n41 - n13 * n24 * n41 - n14 * n21 * n43 + n11 * n24 * n43 + n13 * n21 * n44 - n11 * n23 * n44 ) * det_inv;
		self.elements[ 10 ] = ( n12 * n24 * n41 - n14 * n22 * n41 + n14 * n21 * n42 - n11 * n24 * n42 - n12 * n21 * n44 + n11 * n22 * n44 ) * det_inv;
		self.elements[ 11 ] = ( n13 * n22 * n41 - n12 * n23 * n41 - n13 * n21 * n42 + n11 * n23 * n42 + n12 * n21 * n43 - n11 * n22 * n43 ) * det_inv;

		self.elements[ 12 ] = t14 * det_inv;
		self.elements[ 13 ] = ( n13 * n24 * n31 - n14 * n23 * n31 + n14 * n21 * n33 - n11 * n24 * n33 - n13 * n21 * n34 + n11 * n23 * n34 ) * det_inv;
		self.elements[ 14 ] = ( n14 * n22 * n31 - n12 * n24 * n31 - n14 * n21 * n32 + n11 * n24 * n32 + n12 * n21 * n34 - n11 * n22 * n34 ) * det_inv;
		self.elements[ 15 ] = ( n12 * n23 * n31 - n13 * n22 * n31 + n13 * n21 * n32 - n11 * n23 * n32 - n12 * n21 * n33 + n11 * n22 * n33 ) * det_inv;

	}

	pub fn scale(&mut self, v: &Vector3) {
		let x = v.get_x();
		let y = v.get_y();
		let z = v.get_z();

		self.elements[ 0 ] *= x;
		self.elements[ 4 ] *= y;
		self.elements[ 8 ] *= z;
		self.elements[ 1 ] *= x;
		self.elements[ 5 ] *= y;
		self.elements[ 9 ] *= z;
		self.elements[ 2 ] *= x;
		self.elements[ 6 ] *= y;
		self.elements[ 10 ] *= z;
		self.elements[ 3 ] *= x;
		self.elements[ 7 ] *= y;
		self.elements[ 11 ] *= z;
	}

	pub fn get_max_scale_on_axis(&mut self) -> f32 {
		let scale_x_sq = self.elements[ 0 ] * self.elements[ 0 ] + self.elements[ 1 ] * self.elements[ 1 ] + self.elements[ 2 ] * self.elements[ 2 ];
		let scale_y_sq = self.elements[ 4 ] * self.elements[ 4 ] + self.elements[ 5 ] * self.elements[ 5 ] + self.elements[ 6 ] * self.elements[ 6 ];
		let scale_z_sq = self.elements[ 8 ] * self.elements[ 8 ] + self.elements[ 9 ] * self.elements[ 9 ] + self.elements[ 10 ] * self.elements[ 10 ];
		scale_x_sq.max(scale_y_sq.max(scale_z_sq)).sqrt()
	}

	pub fn make_translation(&mut self, x: f32, y: f32, z: f32) {
		self.set(
			1.0, 0.0, 0.0, x,
			0.0, 1.0, 0.0, y,
			0.0, 0.0, 1.0, z,
			0.0, 0.0, 0.0, 1.0
		);
	}

	pub fn make_rotation_x(&mut self, theta: f32) {
		let c = theta.cos();
		let s = theta.sin();

		self.set(
			1.0, 0.0,  0.0, 0.0,
			0.0, c, -s, 0.0,
			0.0, s,  c, 0.0,
			0.0, 0.0,  0.0, 1.0
		);
	}

	pub fn make_rotation_y(&mut self, theta: f32) {
		let c = theta.cos();
		let s = theta.sin();

		self.set(
			 c, 0.0, s, 0.0,
			 0.0, 1.0, 0.0, 0.0,
			- s, 0.0, c, 0.0,
			 0.0, 0.0, 0.0, 1.0
		);
	}

	pub fn make_rotation_z(&mut self, theta: f32) {
		let c = theta.cos();
		let s = theta.sin();

		self.set(
			c, - s, 0.0, 0.0,
			s,  c, 0.0, 0.0,
			0.0,  0.0, 1.0, 0.0,
			0.0,  0.0, 0.0, 1.0
		);
	}

	pub fn make_rotation_axis(&mut self, axis: &Vector3, angle: f32) {
		let c = angle.cos();
		let s = angle.sin();
		let t = 1.0 - c;
		let x = axis.get_x();
		let y = axis.get_y();
		let z = axis.get_z();
		let tx = t * x;
		let ty = t * y;

		self.set(
			tx * x + c, tx * y - s * z, tx * z + s * y, 0.0,
			tx * y + s * z, ty * y + c, ty * z - s * x, 0.0,
			tx * z - s * y, ty * z + s * x, t * z * z + c, 0.0,
			0.0, 0.0, 0.0, 1.0
		);
	}

	pub fn make_scale(&mut self, x: f32, y: f32, z: f32) {
		self.set(
			x, 0.0, 0.0, 0.0,
			0.0, y, 0.0, 0.0,
			0.0, 0.0, z, 0.0,
			0.0, 0.0, 0.0, 1.0
		);
	}

	pub fn compose(&mut self, position: &Vector3, quaternion: &Quaternion, scale: &Vector3) {
		self.make_rotation_from_quaternion(quaternion);
		self.scale(scale);
		self.set_position(position);
	}

	pub fn decompose(&self, position: &mut Vector3, quaternion: &mut Quaternion, scale: &mut Vector3) {

		let mut vector = Vector3::new();
		let mut matrix = Matrix4::new();

		vector.set( self.elements[ 0 ], self.elements[ 1 ], self.elements[ 2 ] );
		let mut sx = vector.length();
		vector.set( self.elements[ 4 ], self.elements[ 5 ], self.elements[ 6 ] );
		let sy = vector.length();
		vector.set( self.elements[ 8 ], self.elements[ 9 ], self.elements[ 10 ] );
		let sz = vector.length();

		// if determine is negative, we need to invert one scale
		let det = self.determinant();
		if det < 0.0 {
			sx = - sx;
		}

		position.set_x(self.elements[ 12 ]);
		position.set_y(self.elements[ 13 ]);
		position.set_z(self.elements[ 14 ]);

		// scale the rotation part

		(&mut matrix.elements).copy_from_slice( &self.elements ); // at this point matrix is incomplete so we can't use .copy()

		let inv_sx = 1.0 / sx;
		let inv_sy = 1.0 / sy;
		let inv_sz = 1.0 / sz;

		matrix.elements[ 0 ] *= inv_sx;
		matrix.elements[ 1 ] *= inv_sx;
		matrix.elements[ 2 ] *= inv_sx;

		matrix.elements[ 4 ] *= inv_sy;
		matrix.elements[ 5 ] *= inv_sy;
		matrix.elements[ 6 ] *= inv_sy;

		matrix.elements[ 8 ] *= inv_sz;
		matrix.elements[ 9 ] *= inv_sz;
		matrix.elements[ 10 ] *= inv_sz;

		quaternion.set_from_rotation_matrix( &matrix );

		scale.set_x(sx);
		scale.set_y(sy);
		scale.set_z(sz);
	}

	pub fn make_frustum(&mut self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
		let x = 2.0 * near / ( right - left );
		let y = 2.0 * near / ( top - bottom );

		let a = ( right + left ) / ( right - left );
		let b = ( top + bottom ) / ( top - bottom );
		let c = - ( far + near ) / ( far - near );
		let d = - 2.0 * far * near / ( far - near );

		self.elements[ 0 ] = x;
		self.elements[ 4 ] = 0.0;
		self.elements[ 8 ] = a;
		self.elements[ 12 ] = 0.0;
		self.elements[ 1 ] = 0.0;
		self.elements[ 5 ] = y;
		self.elements[ 9 ] = b;
		self.elements[ 13 ] = 0.0;
		self.elements[ 2 ] = 0.0;
		self.elements[ 6 ] = 0.0;
		self.elements[ 10 ] = c;
		self.elements[ 14 ] = d;
		self.elements[ 3 ] = 0.0;
		self.elements[ 7 ] = 0.0;
		self.elements[ 11 ] = - 1.0;
		self.elements[ 15 ] = 0.0;
	}

	pub fn make_perspective(&mut self, fov: f32, aspect: f32, near: f32, far: f32) {
		let ymax = near * (fov.to_radians() * 0.5);
		let ymin = - ymax;
		let xmin = ymin * aspect;
		let xmax = ymax * aspect;

		self.make_frustum(xmin, xmax, ymin, ymax, near, far);
	}

	pub fn make_orthographic(&mut self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
		let w = 1.0 / ( right - left );
		let h = 1.0 / ( top - bottom );
		let p = 1.0 / ( far - near );

		let x = ( right + left ) * w;
		let y = ( top + bottom ) * h;
		let z = ( far + near ) * p;

		self.elements[ 0 ] = 2.0 * w;
		self.elements[ 4 ] = 0.0;
		self.elements[ 8 ] = 0.0;
		self.elements[ 12 ] = - x;
		self.elements[ 1 ] = 0.0;
		self.elements[ 5 ] = 2.0 * h;
		self.elements[ 9 ] = 0.0;
		self.elements[ 13 ] = - y;
		self.elements[ 2 ] = 0.0;
		self.elements[ 6 ] = 0.0;
		self.elements[ 10 ] = - 2.0 * p;
		self.elements[ 14 ] = - z;
		self.elements[ 3 ] = 0.0;
		self.elements[ 7 ] = 0.0;
		self.elements[ 11 ] = 0.0;
		self.elements[ 15 ] = 1.0;
	}

	pub fn equals(&mut self, matrix: &Matrix4) -> bool {
		let me = matrix.get_elements();
		for i in 0..16 {
			if self.elements[ i ] != me[ i ] {
				return false;
			} 
		}
		
		true
	}

	pub fn copy_from_array(&mut self, array: &[f32], offset: Option<usize>) {
		let offset = match offset {
			Some(off) => off,
			None => 0,
		};

		for i in 0..16 {
			self.elements[ i ] = array[ i + offset ];
		}
	}

	pub fn copy_to_array(&self, array: &mut [f32], offset: Option<usize>) {
		let offset = match offset {
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
		array[ offset + 9 ]  = self.elements[ 9 ];
		array[ offset + 10 ] = self.elements[ 10 ];
		array[ offset + 11 ] = self.elements[ 11 ];

		array[ offset + 12 ] = self.elements[ 12 ];
		array[ offset + 13 ] = self.elements[ 13 ];
		array[ offset + 14 ] = self.elements[ 14 ];
		array[ offset + 15 ] = self.elements[ 15 ];
	}
}