// extern crate uuid;

// use ::std::collections::HashSet;
// use ::std::cmp::{Eq, PartialEq};
// use ::std::hash::{Hash, Hasher};
// use ::std::cell::RefCell;
// use ::std::ops::DerefMut;
// use self::uuid::Uuid;

// pub struct EventHandler<T> {
// 	id: Uuid,
// 	handler: RefCell<Box<FnMut(&T)>>
// }

// impl<T> EventHandler<T> {
// 	pub fn new<F>(fun: F) -> EventHandler<T> where F : 'static + FnMut(&T) {
// 		EventHandler {
// 			id: Uuid::new_v4(),
// 			handler: RefCell::new(Box::new(fun))
// 		}
// 	}
// }

// impl<T> PartialEq for EventHandler<T> {
// 	fn eq(&self, other: &EventHandler<T>) -> bool {
// 		self.id == other.id
// 	}
// }

// impl<T> Eq for EventHandler<T> {}

// impl<T> Hash for EventHandler<T> {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		self.id.hash(state);
// 	}
// }

// pub struct EventDispatcher<T> {
// 	listeners: HashSet<Box<EventHandler<T>>>
// }

// impl<T> EventDispatcher<T> {
// 	pub fn new() -> EventDispatcher<T> {
// 		EventDispatcher {
// 			listeners: HashSet::new()
// 		}
// 	}

// 	pub fn add_event_listeners(&mut self, listener: Box<EventHandler<T>>) -> bool {
// 		self.listeners.insert(listener)
// 	}

// 	pub fn has_event_listener(&self, listener: Box<EventHandler<T>>) -> bool {
// 		self.listeners.contains(&listener)
// 	}

// 	pub fn remove_event_listener(&mut self, listener: Box<EventHandler<T>>) -> bool {
// 		self.listeners.remove(&listener)
// 	}

// 	pub fn dispatch_event(&mut self, event: &T) {
// 		for h in self.listeners.iter() {
// 			let mut handler = h.handler.borrow_mut();
// 			handler.deref_mut()(event);
// 		}
// 	}
// }



// #[cfg(test)]
// mod tests {
// 	use super::{EventDispatcher, EventHandler};

// 	#[derive(Debug)]
// 	enum TestEvent {
// 		Event1,
// 		Event2(&'static str),
// 		Event3 {
// 			thing: i32
// 		},
// 	}

// 	#[test]
// 	fn it_works() {
// 		let mut dispatcher = EventDispatcher::new();
// 		let mut thing = 0;
// 		let mut handler1 = EventHandler::new(move |t: &TestEvent| &thing = 1);
// 		dispatcher.add_event_listeners(Box::new(handler1));
// 		// assert!(dispatcher.has_event_listener(Box::new(handler1)));
// 		let event1 = TestEvent::Event1;
// 		dispatcher.dispatch_event(&event1);
// 		assert!(thing == 0);
		
		
// 	}
// }
