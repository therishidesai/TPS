extern crate tps;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::VecDeque;

/*
TPS implemtnation idea

 - publish(topic, data)
     puts the topic and data pair on a publish workqueue
 - subscribe(topic, callback)
     registers the callback function with a topic
 - publisher()
     function/thread that pulls from the publish workqueue
     and puts subscriber callbacks on the subscriber
     workqueue
 - subscribers()
     threadpool that pulls from the subscriber workqueue
     and executes the callback with the published data
 */

// Callback experiments

fn apply<F>(f: F) where
	F: Fn() {
	f();
}

fn simple_callback1(x: i32) {
    println!("hello world, {} is a number", x);
}

fn main() {
	let buf = Arc::new(Mutex::new(VecDeque::new()));
	let stop_flag = Arc::new(AtomicBool::new(false));
	
	let mut children = vec![];
	let workers = 3;

	for worker in 0..workers{
		// ref count for the workqueue for this worker thread
		let stop_flagn = Arc::clone(&stop_flag);
		let bufn = buf.clone();
		children.push(thread::spawn(move ||{
			loop {
				let mut workqueue = bufn.lock().unwrap();
				let work_maybe = workqueue.pop_front();
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
		}));
		
	}

	for x in 0..100 {
		// Capture `x` into an anonymous type and implement
		// `Fn` for it. Store it in `print`.
		let mut workqueue = buf.lock().unwrap();
		workqueue.push_back(move || simple_callback1(x));
	}

	stop_flag.swap(true, Ordering::Relaxed);

	for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
