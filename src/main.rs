extern crate tps;

use rand::Rng;
use std::{thread, time};
use tps::threadpool::ThreadPool;

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

// ThreadPool Experiment

fn simple_callback1(x: i32) {
    println!("hello world, {} is a number", x);
	// randomly sleep for some seconds, mimic long running functions
	let mut rng = rand::thread_rng();
	let mul = rng.gen_range(1, 5);
	thread::sleep(time::Duration::from_millis(1000 * mul));
}

fn main() {
	let mut tp = ThreadPool::new(5);
	
	for x in 0..100 {
		// Capture `x` into an anonymous type and implement
		// `Fn` for it. Store it in `print`.
		tp.push_work(move || simple_callback1(x));
	}

	tp.stop();
}
