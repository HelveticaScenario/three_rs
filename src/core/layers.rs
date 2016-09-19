#[derive(Debug)]
pub struct Layers {
	mask: u64,
}

impl Layers {
	pub fn new() -> Layers {
		Layers {
			mask: 1u64
		}
	}

	pub fn set(&mut self, channel: u64) {
		self.mask = 1u64 << channel;
	}

	pub fn enable(&mut self, channel: u64) {
		self.mask |= 1u64 << channel;
	}

	pub fn toggle(&mut self, channel: u64) {
		self.mask ^= 1u64 << channel;
	}

	pub fn disable(&mut self, channel: u64) {
		self.mask &= ! (1u64 << channel);
	}

	pub fn test(&self, layers: &Layers) -> bool {
		(self.mask & layers.mask) != 0u64
	}
}