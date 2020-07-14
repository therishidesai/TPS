use std::collections::HashMap;

pub mod threadpool;

pub struct TPS {
	topics: HashMap<&'static str, Vec<fn(i32)>>,
	thread_pool: threadpool::ThreadPool
}

impl TPS {
	pub fn new(pool_size: usize) -> TPS {
		let thread_pool = threadpool::ThreadPool::new(pool_size);
		let topics = HashMap::new();
		return TPS {
			topics,
			thread_pool,
		}
	}

	pub fn shutdown(&mut self) {
		self.thread_pool.stop();
	}

	pub fn register_topic(&mut self, topic: &'static str) {
		self.topics.insert(topic, Vec::new());
	}
	pub fn register_subscriber(&mut self, topic: &str, f: fn(i32)) {
		self.topics.get_mut(topic).unwrap().push(f);
	}

	pub fn publish(&mut self, topic: &str, data: i32) {
		let callbacks = self.topics.get_mut(topic).unwrap();
		for i in 0..callbacks.len() {
			let f = callbacks[i];
			self.thread_pool.push_work(move || f(data));
		}
	}
}
