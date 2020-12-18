use std::sync::{Arc, Weak, Mutex};
use std::cell::RefCell;

pub struct SignalBase<T,R> {
    next_id : usize,
    slots : Vec<(usize,Slot<T, R>)>,
}

#[derive(Clone)]
pub struct Signal<T,R> {
    base : Arc<Mutex<RefCell<SignalBase<T,R>>>>,
}

impl<T,R> Signal<T,R> {
    pub fn new() -> Signal<T,R> {
        Signal {
            base : Arc::new(Mutex::new(RefCell::new(SignalBase {
                next_id : 0,
                slots : Vec::new()
            })))
        }
    }

    pub fn connect<Func>(&mut self,func : Func) -> Connection<T,R>
        where Func : 'static + FnMut(T) -> R + Send {
        if let Ok(guard) = self.base.lock() {
            let mut base = guard.borrow_mut();
            let slot = Slot {
                function: Box::new(func),
                return_value: Option::None,
            };
            let connection = Connection {
                signal: Arc::downgrade(&self.base),
                id: base.next_id
            };
            base.slots.push((connection.id, slot));
            base.next_id += 1;
            connection
        }else{
            Connection{
                signal : Weak::new(),
                id : 0
            }
        }
    }

}


impl<T : Copy,R> Signal<T,R>{
    pub fn emit(&mut self,value : T){
        if let Ok(guard) = self.base.lock() {
            let mut base = guard.borrow_mut();
            for (_, slot) in base.slots.iter_mut() {
                slot.emit(value);
            }
        }
    }
}

impl<T : Clone,R> Signal<T,R>{
    pub fn emit_clone(&mut self,value : T){
        if let Ok(guard) = self.base.lock() {
            let mut base = guard.borrow_mut();
            for (_, slot) in base.slots.iter_mut() {
                slot.emit(value.clone());
            }
        }
    }
}

#[derive(Clone)]
pub struct Connection<T,R>{
    signal : Weak<Mutex<RefCell<SignalBase<T,R>>>>,
    id : usize,
}

impl<T,R> Connection<T,R> {
    pub fn guard(self) -> Guard<T,R>{
        Guard{
            connection : self
        }
    }

    pub fn disconnect(&mut self){
        if let Some(signal) = self.signal.upgrade(){
            if let Ok(guard) = (*signal).lock() {
                let mut signal_base = guard.borrow_mut();
                let mut found = Option::None;
                for (index, (id, _)) in signal_base.slots.iter().enumerate() {
                    if *id == self.id {
                        found = Option::Some(index);
                        break;
                    }
                }
                if let Some(index) = found {
                    signal_base.slots.remove(index);
                }
            }
        }
    }

    pub fn connected(&self) -> bool {
        self.signal.upgrade().is_some()
    }

}

pub struct Guard<T,R>{
    connection : Connection<T,R>
}

impl<T,R> Guard<T,R> {
    pub fn connected(&self) -> bool {
        self.connection.connected()
    }
}

impl<T,R> Drop for Guard<T,R> {
    fn drop(&mut self) {
        self.connection.disconnect()
    }
}

struct Slot<T,R>{
    function : Box<dyn FnMut(T) -> R + Send>,
    return_value : Option<R>
}

impl<T,R> Slot<T,R>{
    fn emit(&mut self,value : T){
        let value = (*self.function)(value);
        self.return_value = Some(value);
    }

    #[allow(dead_code)]
    fn return_value(&mut self) -> Option<R>{
        self.return_value.take()
    }
}
