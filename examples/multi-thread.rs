extern crate xrsignal;

use xrsignal::Signal;
use std::thread;

fn main(){
    let mut sig = Signal::new();
    sig.connect(|str|println!("{}",str));

    let mut sig1 = sig.clone();
    let t1 = thread::spawn(move || {
        sig1.emit("t1 Emit");
        thread::sleep(std::time::Duration::from_secs(1));
        sig1.emit("t1 Emit");
    });

    let mut sig2 = sig.clone();
    let t2 = thread::spawn(move || {
        sig2.emit("t2 Emit");
    });

    t2.join().unwrap();
    t1.join().unwrap();
}