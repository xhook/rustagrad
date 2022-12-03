use std::collections::HashMap;

pub struct Value {
    data: f32
}

pub struct Session {
    values: HashMap<u64, Value>
}

impl Session {
    fn new() -> Session {
        Session {
            values: Default::default()
        }
    }

    fn add(&mut self, left: u64, right: u64, result: u64) {
        let l = match self.values.get(&left) {
            Some(v) => v.data,
            None => panic!("boo")
        };
        let r = match self.values.get(&right) {
            Some(v) => v.data,
            None => panic!("boo")
        };
        let res = match self.values.get_mut(&result) {
            Some(v) => v,
            None => panic!("boo")
        };
        res.data = l + r;
    }
}

pub fn test() {
    let mut s = Session::new();
    let a = Value { data: 2f32 };
    let b = Value { data: 2f32 };
    let c = Value { data: 0f32 };
    s.values.insert(0, a);
    s.values.insert(1, b);
    s.values.insert(2, c);
    s.add(0, 1, 2);
    match s.values.get(&2) {
        Some(v) => println!("c = {}", v.data),
        None => panic!("bar")
    }
}