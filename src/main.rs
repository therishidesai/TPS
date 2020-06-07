use std::thread;

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

fn run_cb<F>(f: F, x: i32) where
	F: Fn(i32) {
	f(x)
}
fn simple_callback(x: i32) {
    println!("hello world! {}", x);
}

fn simple_callback1(x: i32) {
    println!("hello world, {} is a number", x);
}

fn main() {
	// run_cb(simple_callback, 1);
	// run_cb(simple_callback1, 2);
	let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || simple_callback1(x);

    apply(print);
}
