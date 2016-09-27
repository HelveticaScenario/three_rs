extern crate uuid;
use std::rc::{Rc, Weak};
use std::cell::{Ref, RefMut, RefCell};
use std::cmp::{Eq, PartialEq};
use self::uuid::Uuid;
use super::super::math::vector3::Vector3;
use super::super::math::quaternion::Quaternion;
use super::super::math::matrix4::Matrix4;
use super::super::math::matrix3::Matrix3;
use super::super::math::euler::Euler;
use super::layers::Layers;

pub static mut DEFAULT_UP: Vector3 = Vector3 {
	x: 0.0,
	y: 1.0,
	z: 0.0,		
};
pub static mut DEFAULT_MATRIX_AUTO_UPDATE: bool = true;

#[derive(Debug, Clone)]
pub struct Object3D {
	_self: Option<Weak<RefCell<Object3D>>>,
	uuid: Uuid,
	name: &'static str,
	children: Vec<Rc<RefCell<Object3D>>>,
	up: Vector3,
	position: Vector3,
	quaternion: Quaternion,
	scale: Vector3,
	matrix_auto_update: bool,
	matrix_world_needs_update: bool,
	model_view_matrix: Matrix4,
	normal_matrix: Matrix3,
	matrix: Matrix4,
	matrix_world: Matrix4,
	layers: Layers,
	visible: bool,
	cast_shadow: bool,
	receive_shadow: bool,
	frustum_culled: bool,
	render_order: u32,
	parent: Option<Rc<RefCell<Object3D>>>,
}

impl PartialEq for Object3D {
	 fn eq(&self, other: &Object3D) -> bool {
		 self.uuid == other.uuid
	 }
}

impl Object3D {
	pub fn new() -> Rc<RefCell<Object3D>> {
		let mut o = Object3D {
			_self: Option::None,
			uuid: Uuid::new_v4(),
			name: "",
			children: vec![],
			up: unsafe {DEFAULT_UP},
			position: Vector3::new(),
			quaternion: Quaternion::new(),
			scale: Vector3 {
				x: 1.0,
				y: 1.0,
				z: 1.0,
			},
			model_view_matrix: Matrix4::new(),
			normal_matrix: Matrix3::new(),
			matrix: Matrix4::new(),
			matrix_world: Matrix4::new(),
			matrix_auto_update: unsafe {DEFAULT_MATRIX_AUTO_UPDATE},
			matrix_world_needs_update: false,
			layers: Layers::new(),
			visible: true,
			cast_shadow: false,
			receive_shadow: false,
			frustum_culled: true,
			render_order: 0,
			parent: Option::None,
		};
		let rc = Rc::new(RefCell::from(o));
		let weak = Rc::downgrade(&rc);
		rc.borrow_mut()._self = Some(weak);
		rc
	}

	pub fn apply_matrix(&mut self, matrix: &Matrix4) {
		let m = self.matrix;
		self.matrix.multiply_matrices(matrix, &m);
		let mut p = self.position;
		let mut q = self.quaternion;
		let mut s = self.scale;
		self.matrix.decompose(&mut p, &mut q, &mut s);
		self.position = p;
		self.quaternion = q;
		self.scale = s;
	}

	pub fn set_rotation_from_axis_angle(&mut self, axis: &Vector3, angle: f32) {
		self.quaternion.set_from_axis_angle(axis, angle);
	}

	pub fn set_rotation_from_euler(&mut self, euler: &Euler) {
		self.quaternion.set_from_euler(euler);
	}

	pub fn set_rotation_from_matrix(&mut self, m: &Matrix4) {
		self.quaternion.set_from_rotation_matrix(m);
	}

	pub fn set_rotation_from_quaternion(&mut self, q: &Quaternion) {
		self.quaternion.copy(q);
	}

	pub fn rotate_on_axis(&mut self, axis: &Vector3, angle: f32) {
		let mut q1 = Quaternion::new();
		q1.set_from_axis_angle(axis, angle);
		self.quaternion.multiply(&q1);
	}

	pub fn rotate_x(&mut self, angle: f32) {
		let v1 = Vector3 {
			x: 1.0,
			y: 0.0,
			z: 0.0,
		};
		self.rotate_on_axis(&v1, angle);
	}

	pub fn rotate_y(&mut self, angle: f32) {
		let v1 = Vector3 {
			x: 0.0,
			y: 1.0,
			z: 0.0,
		};
		self.rotate_on_axis(&v1, angle);
	}

	pub fn rotate_z(&mut self, angle: f32) {
		let v1 = Vector3 {
			x: 0.0,
			y: 0.0,
			z: 1.0,
		};
		self.rotate_on_axis(&v1, angle);
	}

	pub fn translate_on_axis(&mut self, axis: &Vector3, distance: f32) {
		let mut v1 = Vector3::new();
		v1.copy(axis);
		v1.apply_quaternion(&self.quaternion);
		v1.multiply_scalar(distance);
		self.position.add(&v1);
	}

	pub fn translate_x(&mut self, distance: f32) {
		let v1 = Vector3 {
			x: 1.0,
			y: 0.0,
			z: 0.0,
		};
		self.translate_on_axis(&v1, distance);
	}

	pub fn translate_y(&mut self, distance: f32) {
		let v1 = Vector3 {
			x: 0.0,
			y: 1.0,
			z: 0.0,
		};
		self.translate_on_axis(&v1, distance);
	}

	pub fn translate_z(&mut self, distance: f32) {
		let v1 = Vector3 {
			x: 0.0,
			y: 0.0,
			z: 1.0,
		};
		self.translate_on_axis(&v1, distance);
	}

	pub fn local_to_world(&self, vector: &mut Vector3) {
		vector.apply_matrix4(&self.matrix_world);
	}

	pub fn world_to_local(&self, vector: &mut Vector3) {
		let mut m1 = Matrix4::new();
		m1.get_inverse(&self.matrix_world, false);
		vector.apply_matrix4(&m1);
	}

	pub fn look_at(&mut self, vector: &Vector3) {
		let mut m1 = Matrix4::new();
		m1.look_at(vector, &self.position, &self.up);
		self.quaternion.set_from_rotation_matrix(&m1);
	}

	pub fn add(&mut self, object: Rc<RefCell<Object3D>>) {
		let ref mut s = self._self;
		let self_rc = s.as_mut().unwrap().upgrade().unwrap();
		object.borrow_mut().parent = Some(self_rc);
		self.children.push(object);
	}
}