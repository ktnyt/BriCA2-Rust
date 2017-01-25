use port::Port;
use unit::*;
use std::collections::HashMap;
use std::sync::Arc;
use af;

pub mod pipe;

pub trait Component : Unit {
    fn input(&mut self);
    fn output(&mut self);
    fn fire(&mut self);
}

pub struct ComponentStruct {
    unit: UnitStruct,
    inputs: HashMap<String, Arc<af::Array>>,
    outputs: HashMap<String, Arc<af::Array>>,
}

impl ComponentStruct {
    pub fn new() -> Self {
        ComponentStruct {
            unit: UnitStruct::new(),
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }
}

impl ComponentStruct {
    pub fn input(&mut self) {
        for (key, port) in self.unit.get_in_ports() {
            self.inputs.insert(key.clone(), port.read());
        }
    }

    pub fn output(&mut self) {
        for (key, port) in self.unit.get_out_ports() {
            match self.outputs.get(key) {
                Some(x) => port.write(x.clone()),
                None    => panic!("Output for key {} does not exist", key),
            }
        }
    }
}

impl Unit for ComponentStruct {
    fn make_in_port(&mut self, key: String, dims: af::Dim4) {
        self.unit.make_in_port(key.clone(), dims);
        self.inputs.insert(key.clone(), Arc::new(af::constant(0.0, dims)));
    }

    fn remove_in_port(&mut self, key: String) {
        self.unit.remove_in_port(key.clone());
        self.inputs.remove(&key);
    }

    fn make_out_port(&mut self, key: String, dims: af::Dim4) {
        self.unit.make_out_port(key.clone(), dims);
        self.outputs.insert(key.clone(), Arc::new(af::constant(0.0, dims)));
    }

    fn remove_out_port(&mut self, key: String) {
        self.unit.remove_out_port(key.clone());
        self.outputs.remove(&key);
    }

    delegate! {
        for unit;
        fn get_in_port(&mut self, key: String) -> &mut Port;
        fn get_in_ports(&mut self) -> &mut HashMap<String, Port>;
        fn get_out_port(&mut self, key: String) -> &mut Port;
        fn get_out_ports(&mut self) -> &mut HashMap<String, Port>;
        fn connect(&mut self, from: String, other: &mut Unit, to: String);
    }
}
