use std::collections::HashMap;
use std::sync::Arc;

use druid::{Data, Lens, Widget};
#[derive(Clone, Data)]
pub enum Packet {
    Float(f64),
    Bool(bool),
    String(String),
}

#[derive(Clone)]
pub struct VertexData {
    inputs: Arc<HashMap<&'static str, Packet>>,
    outputs: Arc<HashMap<&'static str, Packet>>,
    generate_widget: fn(data: &VertexData) -> Box<dyn Widget<VertexData>>,
}

impl VertexData {
    pub fn new(
        inputs: HashMap<&'static str, Packet>,
        outputs: HashMap<&'static str, Packet>,
        generate_widget: fn(data: &VertexData) -> Box<dyn Widget<VertexData>>,
    ) -> Self {
        VertexData {
            inputs: Arc::new(inputs),
            outputs: Arc::new(outputs),
            generate_widget,
        }
    }
    pub fn generate_widget(&self) -> Box<dyn Widget<VertexData>> {
        (self.generate_widget)(&self)
    }
}

impl Data for VertexData {
    fn same(&self, other: &Self) -> bool {
        self.inputs.same(&other.inputs) && self.outputs.same(&other.outputs)
    }
}

// The following lenses need to be replaced with a macro on the enum I think.
pub struct FloatInputLens(pub &'static str);

impl Lens<VertexData, f64> for FloatInputLens {
    fn with<R, F: FnOnce(&f64) -> R>(&self, data: &VertexData, f: F) -> R {
        let input = data.inputs.get(&self.0).cloned().unwrap();
        match input {
            Packet::Float(value) => f(&value),
            _ => panic!("input was not a Float"),
        }
    }

    fn with_mut<R, F: FnOnce(&mut f64) -> R>(&self, data: &mut VertexData, f: F) -> R {
        let mut input = match data.inputs.get(&self.0).cloned().unwrap() {
            Packet::Float(value) => value,
            _ => panic!("input was not a Float"),
        };
        let old = input.clone();
        let result = f(&mut input);
        let changed = !input.same(&old);
        if changed {
            Arc::make_mut(&mut data.inputs).insert(self.0, Packet::Float(input));
        }
        result
    }
}

pub struct StringInputLens(pub &'static str);

impl Lens<VertexData, String> for StringInputLens {
    fn with<R, F: FnOnce(&String) -> R>(&self, data: &VertexData, f: F) -> R {
        let input = data.inputs.get(&self.0).cloned().unwrap();
        match input {
            Packet::String(value) => f(&value),
            _ => panic!("input was not a Float"),
        }
    }

    fn with_mut<R, F: FnOnce(&mut String) -> R>(&self, data: &mut VertexData, f: F) -> R {
        let mut input = match data.inputs.get(&self.0).cloned().unwrap() {
            Packet::String(value) => value,
            _ => panic!("input was not a Float"),
        };
        let old = input.clone();
        let result = f(&mut input);
        let changed = !input.same(&old);
        if changed {
            Arc::make_mut(&mut data.inputs).insert(self.0, Packet::String(input));
        }
        result
    }
}

pub struct BoolInputLens(pub &'static str);

impl Lens<VertexData, bool> for BoolInputLens {
    fn with<R, F: FnOnce(&bool) -> R>(&self, data: &VertexData, f: F) -> R {
        let input = data.inputs.get(&self.0).cloned().unwrap();
        match input {
            Packet::Bool(value) => f(&value),
            _ => panic!("input was not a Float"),
        }
    }

    fn with_mut<R, F: FnOnce(&mut bool) -> R>(&self, data: &mut VertexData, f: F) -> R {
        let mut input = match data.inputs.get(&self.0).cloned().unwrap() {
            Packet::Bool(value) => value,
            _ => panic!("input was not a Float"),
        };
        let old = input.clone();
        let result = f(&mut input);
        let changed = !input.same(&old);
        if changed {
            Arc::make_mut(&mut data.inputs).insert(self.0, Packet::Bool(input));
        }
        result
    }
}
