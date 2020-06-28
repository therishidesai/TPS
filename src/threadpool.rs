use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::VecDeque;


pub struct ThreadPool {
	workers: Vec<Option<std::thread::JoinHandle<()>>>,
	stop_flag: Arc<AtomicBool>,
	workqueue: Arc<Mutex<VecDeque<Box<Fn() + Send + 'static>>>>
}

fn apply<F>(f: F) where
	F: Fn() {
	f();
}

impl ThreadPool {
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);
		let mut workers = Vec::with_capacity(size);
		let stop_flag = Arc::new(AtomicBool::new(false));
		let workqueue = Arc::new(Mutex::new(VecDeque::new()));

		for worker in 0..size {
			// ref count for the workqueue for this worker thread
			let stop_flagn = Arc::clone(&stop_flag);
			let bufn = workqueue.clone();
			workers.push(Some(thread::spawn(move ||{
				loop {
					let mut wq = bufn.lock().unwrap();
					let work_maybe = wq.pop_front();
					drop(wq);
					match work_maybe {
						None => {
							if stop_flagn.load(Ordering::Relaxed) {
								break;
							} else {
								continue;
							}
						},
						Some(work) => {
							print!("Thread{}: ", worker);
							apply(work);
						},
					}
				}
			})));
		}

		ThreadPool {
			workers,
			stop_flag,
			workqueue,
		}
	}

	pub fn stop(&mut self) {
		self.stop_flag.swap(true, Ordering::Relaxed);

		for worker in &mut self.workers {
			// Wait for the thread to finish. Returns a result.
			if let Some(thread) = worker.take() {
				thread.join().unwrap();
			}
		}
	}

	pub fn push_work<F>(&self, f: F) where
		F: Fn() + Send + 'static{
		let mut wq = self.workqueue.lock().unwrap();
		wq.push_back(Box::new(f));
	}
}
