use std::collections::{HashMap, HashSet};
use std::ops;
use std::ops::Deref;

use append_only_vec::AppendOnlyVec;
use elsa::FrozenMap;

static mut COUNTER: u64 = 0;

pub struct Session<'a> {
    values: FrozenMap<u64, Box<Value<'a>>>
}

#[derive(Clone)]
pub struct Value<'a> {
    id: u64,
    session: &'a Session<'a>,
    pub data: f32,
    pub grad: f32,
    children: Vec<u64>,
    pub op: &'static str,
}

fn get_or_fail<T>(a: Option<T>) -> T {
    match a {
        Some(x) => x,
        None => panic!("The option is None")
    }
}

fn build_topo2<'a: 't, 't>(v_id: &u64,
                           values: &'a FrozenMap<u64, Box<Value<'a>>>,
                           topo: &'t mut Vec<&'a Value<'a>>,
                           visited: &'t mut HashSet<u64>) {
    let v = get_or_fail(values.get(&v_id));
    if !visited.contains(&v.id) {
        visited.insert(v.id);
        topo.push(v);
        let children = v.children.clone();
        for child_id in children.iter() {
            build_topo2(child_id, values, topo, visited);
        }
    }
}


impl<'a> Session<'a> {
    pub fn start() -> Session<'a> {
        Session {
            values: FrozenMap::new()
        }
    }

    fn get_value(&'a self, id: &u64) -> &'a Value<'a> {
        match self.values.get(&id) {
            Some(v) => v,
            None => panic!("Can't find value with id {}. Impossible!", id)
        }
    }

    fn get_value_mut(&'a mut self, id: &u64) -> &mut Value<'a> {
        match self.values.as_mut().get_mut(&id) {
            Some(v) => v,
            None => panic!("Can't find value with id {}. Impossible!", id)
        }
    }

    pub fn value(&'a self, data: f32) -> &Value {
        let mut value = Value {
            id: 0,
            session: &self,
            data: data,
            grad: 0f32,
            children: Vec::new(),
            op: "",
        };
        self.insert_value(value)
    }

    fn insert_value(&'a self, mut value: Value<'a>) -> &Value<'a> {
        let id = self.next_id();
        value.id = id;
        self.values.insert(value.id, Box::new(value));
        self.get_value(&id)
    }

    pub fn backward(&'a mut self, v: &'a Value<'a>) {
        let mut topo: Vec<&Value<'a>> = Vec::new();
        let mut visited: HashSet<u64> = Default::default();
        let mut value = get_or_fail(self.values.as_mut().get_mut(&v.id));
        value.grad = 1f32;
        build_topo2(&v.id, &self.values, &mut topo, &mut visited);
        for x in topo.iter().rev() {
            match x.op {
                "+" => {
                    let v1 = *x;
                    let left_grad = get_or_fail(self.values.get(&v1.children[0])).grad;
                    let right_grad = get_or_fail(self.values.get(&v1.children[1])).grad;
                    let mut left = get_or_fail(self.values.as_mut().get_mut(&v1.children[0]));
                    left.grad = right_grad + v1.grad;
                    let mut right = get_or_fail(self.values.as_mut().get_mut(&v1.children[1]));
                    right.grad = left_grad + v1.grad;
                }
                // "*" => self.mul_backward(&mut x.clone()),
                "" => continue,
                _ => panic!("Not supported backprop op {}", x.op)
            }
        }
    }

    fn build_topo<'t>(&'a self, v_id: &u64, topo: &'t mut Vec<&'a Value<'a>>, visited: &'t mut HashSet<u64>) {
        let v = self.get_value(v_id);
        if !visited.contains(&v.id) {
            visited.insert(v.id);
            topo.push(v);
            let children = v.children.clone();
            for child_id in children.iter() {
                self.build_topo(child_id, topo, visited);
            }
        }
    }

    fn foo(&'a mut self) {

    }

    fn add_backward(&'a mut self, v: &Value<'a>) {
        let left_grad = get_or_fail(self.values.get(&v.children[0])).grad;
        let right_grad = get_or_fail(self.values.get(&v.children[1])).grad;
        let mut left = get_or_fail(self.values.as_mut().get_mut(&v.children[0]));
        left.grad = right_grad + v.grad;
        let mut right = get_or_fail(self.values.as_mut().get_mut(&v.children[1]));
        right.grad = left_grad + v.grad;
    }

    // fn mul_backward(&'a self, v: &'a mut Value<'a>) {
    //     self.data.push(v.children[0].grad + v.children[1].grad * v.grad);
    //     self.data.push(v.children[1].grad + v.children[0].grad * v.grad);
    //     v.children[0].grad = &self.data[self.data.len()-2];
    //     v.children[1].grad = &self.data[self.data.len()-1];
    // }

    fn next_id(&self) -> u64 {
        unsafe {
            COUNTER += 1;
            COUNTER
        }
    }
}

pub fn add<'a>(a: &'a Value<'a>, b: &'a Value<'a>) -> &'a Value<'a> {
    if a.session as *const _ != b.session as *const _ {
        panic!("Values a and b are from different sessions")
    }
    let value = Value {
        id: 0,
        session: a.session,
        data: a.data + b.data,
        grad: 0f32,
        children: vec![a.id, b.id],
        op: "+"
    };
    a.session.insert_value(value)
}

// pub fn add2(&'a self, a: Value<'a>, b: &Value<'a>) -> Value {
//     let id = self.next_id();
//     self.data.push(a.data + b.data);
//     self.data.push(0f32);
//     let d = &self.data[self.data.len()-2];
//     let g = &self.data[self.data.len()-1];
//     Value { id: id, data: d, grad: g, children: vec![a, b.clone()], op: "+" }
// }
//
// pub fn sub(&'a self, a: &Value<'a>, b: &Value<'a>) -> Value {
//     let minus_one = self.value(-1f32);
//     let minus_b = self.mul2(minus_one, b);
//     let out = self.add2(minus_b, a);
//     out.clone()
// }
//
// pub fn mul(&'a self, a: &Value<'a>, b: &Value<'a>) -> Value {
//     self.data.push(a.data * b.data);
//     self.data.push(0f32);
//     let d = &self.data[self.data.len()-2];
//     let g = &self.data[self.data.len()-1];
//     Value { id: self.next_id(), data: d, grad: g, children: vec![a.clone(), b.clone()], op: "*" }
// }
//
// pub fn mul2(&'a self, a: Value<'a>, b: &Value<'a>) -> Value {
//     self.data.push(a.data * b.data);
//     self.data.push(0f32);
//     let d = &self.data[self.data.len()-2];
//     let g = &self.data[self.data.len()-1];
//     Value { id: self.next_id(), data: d, grad: g, children: vec![a, b.clone()], op: "*" }
// }
//
