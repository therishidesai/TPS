// use std::collections::HashMap;

pub mod threadpool;

// pub struct TPS {
// 	topics: HashMap<&str, Vec<&dyn Fn(i32)>>,
// 	thread_pool: threadpool::ThreadPool
// }

// impl TPS {
// 	pub fn new(pool_size: usize) -> TPS {
// 		let mut thread_pool = threadpool::ThreadPool::new(pool_size);
// 		let mut topics = HashMap::new();
// 		return TPS {
// 			topics,
// 			thread_pool,
// 		}
// 	}

// 	pub fn shutdown(&mut self) {
// 		self.thread_pool.stop();
// 	}

// 	pub fn register_subscriber(&mut self, topic: &str, f: &dyn Fn(i32)) {
// 		self.insert(topic, f);
// 	}

// 	pub fn publish(&mut self, topic: &str, data: )
// }
