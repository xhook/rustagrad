use append_only_vec::AppendOnlyVec;

pub struct Session<'v> {
    values: AppendOnlyVec<Box<Value<'v>>>
}

#[derive(Debug)]
pub struct Value<'v> {
    pub data: f32,
    pub grad: f32,
    children: Vec<Box<&'v Value<'v>>>,
    pub op: &'static str,
}

impl<'v> Session<'v> {
    pub fn new() -> Self {
        Self {
            values: AppendOnlyVec::new()
        }
    }

    pub fn value<'s: 'v>(&'s self, v: f32) -> &'v Value {
        let value = Value {
            data: v,
            grad: 0f32,
            children: vec![],
            op: ""
        };
        let last = self.values.push(Box::new(value));
        &self.values[last]
    }

    // pub fn add(&'a self, a: &'a Value<'a>, b: &'a Value<'a>) -> &'a Value<'a> {
    //     let value = Value {
    //         data: a.data + b.data,
    //         grad: 0f32,
    //         children: vec![Box::new(a), Box::new(b)],
    //         op: "+"
    //     };
    //     let last = self.values.push(Box::new(value));
    //     &self.values[last]
    // }
}