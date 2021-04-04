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
pub struct Node {
    inputs: Arc<HashMap<&'static str, Packet>>,
    outputs: Arc<HashMap<&'static str, Packet>>,
    id: usize,
    generate_widget: fn(data: &Node) -> Box<dyn Widget<Node>>,
}

impl Node {
    pub fn new(
        inputs: HashMap<&'static str, Packet>,
        outputs: HashMap<&'static str, Packet>,
        id: usize,
        generate_widget: fn(data: &Node) -> Box<dyn Widget<Node>>,
    ) -> Self {
        Node {
            inputs: Arc::new(inputs),
            outputs: Arc::new(outputs),
            id,
            generate_widget,
        }
    }

    pub fn id(&self) -> usize {
        self.id.clone()
    }
    pub fn generate_widget(&self) -> Box<dyn Widget<Node>> {
        (self.generate_widget)(&self)
    }
}

impl Data for Node {
    fn same(&self, other: &Self) -> bool {
        self.inputs.same(&other.inputs) && self.outputs.same(&other.outputs)
    }
}

// The following lenses need to be replaced with a macro on the enum I think.
pub struct FloatInputLens(pub &'static str);

impl Lens<Node, f64> for FloatInputLens {
    fn with<R, F: FnOnce(&f64) -> R>(&self, data: &Node, f: F) -> R {
        let input = data.inputs.get(&self.0).cloned().unwrap();
        match input {
            Packet::Float(value) => f(&value),
            _ => panic!("input was not a Float"),
        }
    }

    fn with_mut<R, F: FnOnce(&mut f64) -> R>(&self, data: &mut Node, f: F) -> R {
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

impl Lens<Node, String> for StringInputLens {
    fn with<R, F: FnOnce(&String) -> R>(&self, data: &Node, f: F) -> R {
        let input = data.inputs.get(&self.0).cloned().unwrap();
        match input {
            Packet::String(value) => f(&value),
            _ => panic!("input was not a Float"),
        }
    }

    fn with_mut<R, F: FnOnce(&mut String) -> R>(&self, data: &mut Node, f: F) -> R {
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

impl Lens<Node, bool> for BoolInputLens {
    fn with<R, F: FnOnce(&bool) -> R>(&self, data: &Node, f: F) -> R {
        let input = data.inputs.get(&self.0).cloned().unwrap();
        match input {
            Packet::Bool(value) => f(&value),
            _ => panic!("input was not a Float"),
        }
    }

    fn with_mut<R, F: FnOnce(&mut bool) -> R>(&self, data: &mut Node, f: F) -> R {
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
