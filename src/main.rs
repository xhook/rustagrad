extern crate core;

// use crate::grad_box::{Session};

mod grad_rc;
// mod grad_box;

struct Session {
}

pub struct Value {
    pub data: f32,
}

impl Session {
    pub fn new() -> Self {
        Self {}
    }

    pub fn value(&self, v: f32) {
        let value = Value {
            data: v
        };
        // let last = self.values.push(Box::new(value));
        // &self.values[last]
    }
}

fn main() {
    let s = Session::new();
    {
        let v2 = s.value(20f32);
        let v3 = s.value(30f32);
        let v4 = s.value(40f32);
        let v5 = s.value(50f32);
        // let v6 = s.add(v4, v5);
        // println!("{:?}", v2);
        // println!("{:?}", v3);
        // println!("{:?}", v4);
        // println!("{:?}", v5);
    }
    // println!("{:?}", v6);
}