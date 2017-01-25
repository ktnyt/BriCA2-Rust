use port::Port;
use std::collections::HashMap;
use af;

pub trait Unit {
    fn make_in_port(&mut self, key: String, dims: af::Dim4);
    fn get_in_port(&mut self, key: String) -> &mut Port;
    fn get_in_ports(&mut self) -> &mut HashMap<String, Port>;
    fn remove_in_port(&mut self, key: String);
    fn make_out_port(&mut self, key: String, dims: af::Dim4);
    fn get_out_port(&mut self, key: String) -> &mut Port;
    fn get_out_ports(&mut self) -> &mut HashMap<String, Port>;
    fn remove_out_port(&mut self, key: String);
    fn connect(&mut self, from: String, other: &mut Unit, to: String);
}

pub struct UnitStruct {
    in_ports: HashMap<String, Port>,
    out_ports: HashMap<String, Port>,
}

impl UnitStruct {
    pub fn new() -> Self {
        UnitStruct {
            in_ports: HashMap::new(),
            out_ports: HashMap::new(),
        }
    }
}

impl Unit for UnitStruct {
    fn make_in_port(&mut self, key: String, dims: af::Dim4) {
        self.in_ports.insert(key.clone(), Port::new(dims));
    }

    fn get_in_port(&mut self, key: String) -> &mut Port {
        match self.in_ports.get_mut(&key) {
            Some(x) => x,
            None    => panic!("In port {} does not exist", key),
        }
    }

    fn get_in_ports(&mut self) -> &mut HashMap<String, Port> {
        &mut self.in_ports
    }

    fn remove_in_port(&mut self, key: String) {
        self.in_ports.remove(&key);
    }

    fn make_out_port(&mut self, key: String, dims: af::Dim4) {
        self.out_ports.insert(key.clone(), Port::new(dims));
    }

    fn get_out_port(&mut self, key: String) -> &mut Port {
        match self.out_ports.get_mut(&key) {
            Some(x) => x,
            None    => panic!("Out port {} does not exist", key),
        }
    }

    fn get_out_ports(&mut self) -> &mut HashMap<String, Port> {
        &mut self.out_ports
    }

    fn remove_out_port(&mut self, key: String) {
        self.out_ports.remove(&key);
    }

    fn connect(&mut self, from: String, other: &mut Unit, to: String) {
        let mut in_port = self.get_in_port(from);
        let out_port = other.get_out_port(to);
        in_port.connect(out_port);
    }
}
