use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use append_only_vec::AppendOnlyVec;
use elsa::FrozenMap;

#[derive(Clone)]
pub struct Session {
    values: Vec<Rc<Value>>,
}

#[derive(Clone)]
pub struct Value {
    session: Session,
    pub data: f32,
    pub grad: f32,
    children: Vec<Rc<Value>>,
    pub op: &'static str,
}

impl Session {
    pub fn start() -> Self {
        Self {
            // values: AppendOnlyVec::new()
            values: Vec::new()
        }
    }

    pub fn value(&mut self, data: f32) -> Rc<Value> {
        let value = Value {
            session: self.clone(), // TODO: this is super-duper inefficient
            data: data,
            grad: 0f32,
            children: Vec::new(),
            op: "",
        };
        self.push(value)
    }

    fn push(&mut self, value: Value) -> Rc<Value> {
        let rc = Rc::new(value);
        let rc_clone = rc.clone();
        self.values.push(rc);
        rc_clone
    }

    pub fn backward(&mut self, v: Rc<Value>) {
        let mut topo: Vec<Rc<Value>> = Vec::new();
        let mut visited: HashSet<* const Value> = Default::default();
        self.build_topo(v, &mut topo, &mut visited);
        for v in topo.iter().rev() {
            let x = v.clone();
            match x.op {
                "+" => {
                    let mut left_rc = x.children[0].clone();
                    let mut right_rc = x.children[1].clone();
                    let mut left = left_rc.borrow_mut();
                    let mut right = right_rc.borrow_mut();
                    (**left).grad = right.grad + x.grad;
                    (**right).grad = left.grad + x.grad;
                }
                // "*" => self.mul_backward(&mut x.clone()),
                "" => continue,
                _ => panic!("Not supported backprop op {}", x.op)
            }
        }
    }

    fn build_topo(&self, v: Rc<Value>, topo: &mut Vec<Rc<Value>>, visited: &mut HashSet<* const Value>) {
        let ptr = Rc::as_ptr(&v);
        if !visited.contains(&ptr) {
            visited.insert(ptr);
            topo.push(v.clone());
            for &child in v.children.iter() {
                self.build_topo(child.clone(), topo, visited);
            }
        }
    }

}

pub fn add(a: Rc<Value>, b: Rc<Value>) -> Rc<Value> {
    let value = Value {
        session: a.session.clone(), // TODO: this is super-duper inefficient
        data: a.data + b.data,
        grad: 0f32,
        children: vec![a.clone(), b.clone()],
        op: "+",
    };
    a.session.clone().push(value)
}