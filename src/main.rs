extern crate tps;

use rand::Rng;
use std::{thread, time};
use tps::TPS;
//use tps::threadpool::ThreadPool;

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
    println!("hello I'm a subscriber 1, {} is a number", x);
	// randomly sleep for some seconds, mimic long running functions
	let mut rng = rand::thread_rng();
	let mul = rng.gen_range(1, 5);
	thread::sleep(time::Duration::from_millis(100 * mul));
}

fn simple_callback2(x: i32) {
    println!("hello I'm a subscriber 2, {} is a number", x);
	// randomly sleep for some seconds, mimic long running functions
	let mut rng = rand::thread_rng();
	let mul = rng.gen_range(1, 5);
	thread::sleep(time::Duration::from_millis(100 * mul));
}

fn main() {
	let mut tps = TPS::new(1);
	tps.register_topic("test");
	tps.register_subscriber("test", simple_callback1);
	tps.register_subscriber("test", simple_callback2);

	for x in 0..10 {
		println!("Publishing: {} on test topic", x);
		tps.publish("test", x);
	}
	tps.shutdown();
}
