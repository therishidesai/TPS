use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crossbeam::queue::SegQueue;

pub struct ThreadPool {
	workers: Vec<Option<std::thread::JoinHandle<()>>>,
	stop_flag: Arc<AtomicBool>,
	workqueue: Arc<SegQueue<Box<Fn() + Send + 'static>>>,
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
		let workqueue = Arc::new(SegQueue::new());

		for worker in 0..size {
			// ref count for the workqueue for this worker thread
			let stop_flagn = Arc::clone(&stop_flag);
			let wq = workqueue.clone();
			workers.push(Some(thread::spawn(move ||{
				loop {
					let work_maybe = wq.pop();
					match work_maybe {
						Err(_err) => {
							if stop_flagn.load(Ordering::Relaxed) {
								break;
							} else {
								continue;
							}
						},
						Ok(work) => {
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
		self.workqueue.push(Box::new(f));
	}
}
