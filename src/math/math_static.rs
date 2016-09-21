extern crate uuid;
use self::uuid::Uuid;

pub fn generate_UUID() -> Uuid{
	Uuid::new_v4()
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
	min.max(max.min(value))
}

pub fn euclidean_modulo(n: f32, m: f32) -> f32 {
	((n % m) + m) % m
}

pub fn map_linear(x: f32, a1: f32, a2: f32, b1: f32, b2: f32) -> f32 {
	b1 + ((x - a1) * (b2 - b1) / (a2 - a1))
}

pub fn smoothstep(x: f32, min: f32, max: f32) -> f32 {
	if x <= min {
		return 0.0;
	} else if x >= max {
		return 1.0;
	}

	let x = (x - min) / (max - min);
	x * x * (3.0 - (2.0 * x))
}

pub fn smootherstep(x: f32, min: f32, max: f32) -> f32 {
	if x <= min {
		return 0.0;
	} else if x >= max {
		return 1.0;
	}

	let x = (x - min) / (max - min);
	x * x * x * ( x * ( x * 6.0 - 15.0 ) + 10.0 )
}

pub fn nearest_power_of_two(value: u32) -> u32 {
	2u32.pow(( (value as f32).ln() / ::std::f32::consts::LN_2 ).round() as u32)
}


