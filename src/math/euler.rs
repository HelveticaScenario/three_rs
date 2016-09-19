#[derive(Debug, Clone, Copy)]
pub enum RotationOrders {
	XYZ,
	YZX,
	ZXY,
	XZY,
	YXZ,
	ZYX,
}

pub static mut DEFAULT_ORDER: RotationOrders = RotationOrders::XYZ;

#[derive(Debug, Clone, Copy)]
pub struct Euler {
	x: f32,
	y: f32,
	z: f32,
	order: RotationOrders,
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
}