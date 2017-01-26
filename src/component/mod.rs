use port::Port;
use unit::*;
use std::collections::HashMap;
use std::sync::Arc;
use af;

pub mod constant;
pub mod pipe;
pub mod null;

pub trait Component : Unit {
    fn input(&mut self);
    fn output(&mut self);
    fn get_input(&mut self, key: &str) -> Arc<af::Array>;
    fn get_output(&mut self, key: &str) -> Arc<af::Array>;
    fn fire(&mut self);
}

#[derive(Clone)]
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

    pub fn get_input(&mut self, key: &str) -> Arc<af::Array> {
        match self.inputs.get_mut(&key.to_string()) {
            Some(x) => x.clone(),
            None    => panic!("Input `{}` does not exist", key)
        }
    }

    pub fn get_output(&mut self, key: &str) -> Arc<af::Array> {
        match self.outputs.get_mut(&key.to_string()) {
            Some(x) => x.clone(),
            None    => panic!("Output `{}` does not exist", key)
        }
    }
}

impl Unit for ComponentStruct {
    fn make_in_port(&mut self, key: &str, dims: af::Dim4) {
        self.unit.make_in_port(key, dims);
        self.inputs.insert(key.to_string(), Arc::new(af::constant(0.0, dims)));
    }

    fn remove_in_port(&mut self, key: &str) {
        self.unit.remove_in_port(key);
        self.inputs.remove(&key.to_string());
    }

    fn make_out_port(&mut self, key: &str, dims: af::Dim4) {
        self.unit.make_out_port(key, dims);
        self.outputs.insert(key.to_string(), Arc::new(af::constant(0.0, dims)));
    }

    fn remove_out_port(&mut self, key: &str) {
        self.unit.remove_out_port(key);
        self.outputs.remove(&key.to_string());
    }

    delegate! {
        for unit;
        fn get_in_port(&mut self, key: &str) -> &mut Port;
        fn get_in_ports(&mut self) -> &mut HashMap<String, Port>;
        fn get_out_port(&mut self, key: &str) -> &mut Port;
        fn get_out_ports(&mut self) -> &mut HashMap<String, Port>;
        fn connect(&mut self, from: &str, other: &mut Unit, to: &str);
    }
}

#[test]
fn it_works() {
    use component::constant::Constant;
    use component::pipe::Pipe;
    use component::null::Null;

    af::set_backend(af::Backend::CPU);

    let n_rows: u64 = 5;
    let n_cols: u64 = 3;

    let dims = af::Dim4::new(&[n_rows, n_cols, 1, 1]);
    let ones = af::constant(1.0, dims);

    let mut c0 = Constant::new(ones);
    let mut c1 = Pipe::new(("in", "out"));
    let mut c2 = Null::new();

    c0.make_out_port("out", dims);
    c1.make_in_port("in", dims);
    c1.make_out_port("out", dims);
    c2.make_in_port("in", dims);

    connect(&mut c1, "in", &mut c0, "out");
    connect(&mut c2, "in", &mut c1, "out");

    let a0 = c0.get_out_port("out").read();
    let a1 = c1.get_in_port("in").read();
    let a2 = c1.get_out_port("out").read();
    let a3 = c2.get_in_port("in").read();
    let (r0, _) = af::sum_all(&a0);
    let (r1, _) = af::sum_all(&a1);
    let (r2, _) = af::sum_all(&a2);
    let (r3, _) = af::sum_all(&a3);
    assert_eq!(r0, 0.0);
    assert_eq!(r1, 0.0);
    assert_eq!(r2, 0.0);
    assert_eq!(r3, 0.0);

    let a0 = c0.get_output("out");
    let a1 = c1.get_input("in");
    let a2 = c1.get_output("out");
    let a3 = c2.get_input("in");
    let (r0, _) = af::sum_all(&a0);
    let (r1, _) = af::sum_all(&a1);
    let (r2, _) = af::sum_all(&a2);
    let (r3, _) = af::sum_all(&a3);
    assert_eq!(r0, 0.0);
    assert_eq!(r1, 0.0);
    assert_eq!(r2, 0.0);
    assert_eq!(r3, 0.0);

    c0.input();
    c1.input();
    c2.input();
    c0.fire();
    c1.fire();
    c2.fire();
    c0.output();
    c1.output();
    c2.output();

    let a0 = c0.get_out_port("out").read();
    let a1 = c1.get_in_port("in").read();
    let a2 = c1.get_out_port("out").read();
    let a3 = c2.get_in_port("in").read();
    let (r0, _) = af::sum_all(&a0);
    let (r1, _) = af::sum_all(&a1);
    let (r2, _) = af::sum_all(&a2);
    let (r3, _) = af::sum_all(&a3);
    assert_eq!(r0, 15.0);
    assert_eq!(r1, 15.0);
    assert_eq!(r2, 0.0);
    assert_eq!(r3, 0.0);

    let a0 = c0.get_output("out");
    let a1 = c1.get_input("in");
    let a2 = c1.get_output("out");
    let a3 = c2.get_input("in");
    let (r0, _) = af::sum_all(&a0);
    let (r1, _) = af::sum_all(&a1);
    let (r2, _) = af::sum_all(&a2);
    let (r3, _) = af::sum_all(&a3);
    assert_eq!(r0, 15.0);
    assert_eq!(r1, 0.0);
    assert_eq!(r2, 0.0);
    assert_eq!(r3, 0.0);

    c0.input();
    c1.input();
    c2.input();
    c0.fire();
    c1.fire();
    c2.fire();
    c0.output();
    c1.output();
    c2.output();

    let a0 = c0.get_out_port("out").read();
    let a1 = c1.get_in_port("in").read();
    let a2 = c1.get_out_port("out").read();
    let a3 = c2.get_in_port("in").read();
    let (r0, _) = af::sum_all(&a0);
    let (r1, _) = af::sum_all(&a1);
    let (r2, _) = af::sum_all(&a2);
    let (r3, _) = af::sum_all(&a3);
    assert_eq!(r0, 15.0);
    assert_eq!(r1, 15.0);
    assert_eq!(r2, 15.0);
    assert_eq!(r3, 15.0);

    let a0 = c0.get_output("out");
    let a1 = c1.get_input("in");
    let a2 = c1.get_output("out");
    let a3 = c2.get_input("in");
    let (r0, _) = af::sum_all(&a0);
    let (r1, _) = af::sum_all(&a1);
    let (r2, _) = af::sum_all(&a2);
    let (r3, _) = af::sum_all(&a3);
    assert_eq!(r0, 15.0);
    assert_eq!(r1, 15.0);
    assert_eq!(r2, 15.0);
    assert_eq!(r3, 0.0);

    c0.input();
    c1.input();
    c2.input();
    c0.fire();
    c1.fire();
    c2.fire();
    c0.output();
    c1.output();
    c2.output();

    let a0 = c0.get_out_port("out").read();
    let a1 = c1.get_in_port("in").read();
    let a2 = c1.get_out_port("out").read();
    let a3 = c2.get_in_port("in").read();
    let (r0, _) = af::sum_all(&a0);
    let (r1, _) = af::sum_all(&a1);
    let (r2, _) = af::sum_all(&a2);
    let (r3, _) = af::sum_all(&a3);
    assert_eq!(r0, 15.0);
    assert_eq!(r1, 15.0);
    assert_eq!(r2, 15.0);
    assert_eq!(r3, 15.0);

    let a0 = c0.get_output("out");
    let a1 = c1.get_input("in");
    let a2 = c1.get_output("out");
    let a3 = c2.get_input("in");
    let (r0, _) = af::sum_all(&a0);
    let (r1, _) = af::sum_all(&a1);
    let (r2, _) = af::sum_all(&a2);
    let (r3, _) = af::sum_all(&a3);
    assert_eq!(r0, 15.0);
    assert_eq!(r1, 15.0);
    assert_eq!(r2, 15.0);
    assert_eq!(r3, 15.0);
}
