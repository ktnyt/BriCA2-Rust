use port::Port;
use std::collections::HashMap;
use af;

pub trait Unit {
    fn make_in_port(&mut self, key: &str, dims: af::Dim4);
    fn get_in_port(&mut self, key: &str) -> Result<&mut Port, String>;
    fn get_in_ports(&mut self) -> &mut HashMap<String, Port>;
    fn remove_in_port(&mut self, key: &str);
    fn alias_in_port(&mut self, from: &str, other: &mut Unit, to: &str);
    fn make_out_port(&mut self, key: &str, dims: af::Dim4);
    fn get_out_port(&mut self, key: &str) -> Result<&mut Port, String>;
    fn get_out_ports(&mut self) -> &mut HashMap<String, Port>;
    fn remove_out_port(&mut self, key: &str);
    fn alias_out_port(&mut self, from: &str, other: &mut Unit, to: &str);
    fn connect(&mut self, from: &str, other: &mut Unit, to: &str);
}

#[derive(Clone)]
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
    fn make_in_port(&mut self, key: &str, dims: af::Dim4) {
        self.in_ports.insert(key.to_string(), Port::new(dims));
    }

    fn get_in_port(&mut self, key: &str) -> Result<&mut Port, String> {
        match self.in_ports.get_mut(&key.to_string()) {
            Some(x) => Ok(x),
            None    => Err(format!("In port `{}` does not exist", key)),
        }
    }

    fn get_in_ports(&mut self) -> &mut HashMap<String, Port> {
        &mut self.in_ports
    }

    fn remove_in_port(&mut self, key: &str) {
        self.in_ports.remove(&key.to_string());
    }

    fn alias_in_port(&mut self, from: &str, other: &mut Unit, to: &str) {
        let mut from_port = self.get_in_port(from).unwrap();
        let to_port = other.get_in_port(to).unwrap();
        from_port.entangle(to_port);
    }

    fn make_out_port(&mut self, key: &str, dims: af::Dim4) {
        self.out_ports.insert(key.to_string(), Port::new(dims));
    }

    fn get_out_port(&mut self, key: &str) -> Result<&mut Port, String> {
        match self.out_ports.get_mut(&key.to_string()) {
            Some(x) => Ok(x),
            None    => Err(format!("Out port `{}` does not exist", key)),
        }
    }

    fn get_out_ports(&mut self) -> &mut HashMap<String, Port> {
        &mut self.out_ports
    }

    fn remove_out_port(&mut self, key: &str) {
        self.out_ports.remove(&key.to_string());
    }

    fn alias_out_port(&mut self, from: &str, other: &mut Unit, to: &str) {
        let mut from_port = self.get_out_port(from).unwrap();
        let to_port = other.get_out_port(to).unwrap();
        from_port.entangle(to_port);
    }

    fn connect(&mut self, from: &str, other: &mut Unit, to: &str) {
        let mut in_port = self.get_in_port(from).unwrap();
        let out_port = other.get_out_port(to).unwrap();
        in_port.entangle(out_port);
    }
}

pub fn connect(from_unit: &mut Unit, from_port: &str, to_unit: &mut Unit, to_port: &str) {
    from_unit.connect(from_port, to_unit, to_port);
}
