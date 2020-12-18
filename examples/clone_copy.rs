extern crate xrsignal;

use xrsignal::Signal;

fn main(){
    let mut sig = Signal::new();
    sig.connect(|str| println!("str:{}",str));
    sig.emit_clone("asdsad".to_owned());
    // sig.emit("asdsad".to_owned()); //error here :trait 'Copy' is not satisfied
}