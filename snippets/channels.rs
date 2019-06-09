use std::sync::mpsc::channel;
use std::thread;

let (sender, receiver) = channel();

thread::spawn(move|| {
    sender.send(expensive_computation()).unwrap();
});

println!("{:?}", receiver.recv().unwrap());
