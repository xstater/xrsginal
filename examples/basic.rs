extern crate xrsignal;

use xrsignal::Signal;

fn main(){
    let mut sig = Signal::new();
    sig.connect(|x|println!("1:{}",x));
    let mut slot = sig.connect(|x|println!("2:{}",x));
    sig.emit("emit 1st");
    slot.disconnect();
    sig.emit("emit 2nd");
    {
        let _guard = sig.connect(|x|println!("3:{}",x)).guard();
        sig.emit("emit 3rd");
    }
    sig.emit("emit 4th");
}
