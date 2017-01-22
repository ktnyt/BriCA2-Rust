use port::Port;
use std::collections::HashMap;
use std::sync::Arc;
use af;

pub trait Component {
    fn make_in_port(&mut self, key: String, dims: af::Dim4);
    fn get_in_port(&mut self, key: String) -> &mut Port;
    fn make_out_port(&mut self, key: String, dims: af::Dim4);
    fn get_out_port(&mut self, key: String) -> &mut Port;
    fn input(&mut self);
    fn output(&mut self);
    fn fire(&mut self);
}

struct ComponentBase {
    in_ports:  HashMap<String, Port>,
    out_ports: HashMap<String, Port>,
    inputs:  HashMap<String, Arc<af::Array>>,
    outputs: HashMap<String, Arc<af::Array>>,
}

impl ComponentBase {
    pub fn new() -> Self {
        ComponentBase {
            in_ports:  HashMap::new(),
            out_ports: HashMap::new(),
            inputs:  HashMap::new(),
            outputs: HashMap::new(),
        }
    }
}

impl ComponentBase {
    pub fn make_in_port(&mut self, key: String, dims: af::Dim4) {
        self.in_ports.insert(key.clone(), Port::new(dims));
        self.inputs.insert(key.clone(), Arc::new(af::constant(0.0, dims)));
    }

    pub fn get_in_port(&mut self, key: String) -> &mut Port {
        match self.in_ports.get_mut(&key) {
            Some(x) => x,
            None    => panic!("In port {} does not exist", key),
        }
    }

    pub fn make_out_port(&mut self, key: String, dims: af::Dim4) {
        self.out_ports.insert(key.clone(), Port::new(dims));
        self.outputs.insert(key.clone(), Arc::new(af::constant(0.0, dims)));
    }

    pub fn get_out_port(&mut self, key: String) -> &mut Port {
        match self.out_ports.get_mut(&key) {
            Some(x) => x,
            None    => panic!("Out port {} does not exist", key),
        }
    }

    pub fn connect(&mut self, from: String, other: &mut Component, to: String) {
        let mut in_port = self.get_in_port(from);
        let out_port = other.get_out_port(to);
        in_port.connect(out_port);
    }

    pub fn input(&mut self) {
        for (key, port) in &self.in_ports {
            self.inputs.insert(key.clone(), port.read());
        }
    }

    pub fn output(&mut self) {
        for (key, port) in &mut self.out_ports {
            match self.outputs.get(key) {
                Some(x) => port.write(x.clone()),
                None    => panic!("Output for key {} does not exist", key),
            }
        }
    }
}

macro_rules! component {
    ( $name:ident, $param:ident : $ty:ty; $func:expr ) => {
        pub struct $name {
            base: ComponentBase,
            $param: $ty,
        }

        impl $name {
            fn new($param: $ty) -> Self {
                $name {
                    base: ComponentBase::new(),
                    $param: $param,
                }
            }
        }

        impl Component for $name {
            fn make_in_port(&mut self, key: String, dims: af::Dim4) {
                self.base.make_in_port(key, dims);
            }

            fn get_in_port(&mut self, key: String) -> &mut Port {
                self.base.get_in_port(key)
            }

            fn make_out_port(&mut self, key: String, dims: af::Dim4) {
                self.base.make_out_port(key, dims);
            }

            fn get_out_port(&mut self, key: String) -> &mut Port {
                self.base.get_out_port(key)
            }

            fn input(&mut self) {
                self.base.input();
            }

            fn output(&mut self) {
                self.base.output();
            }

            fn fire(&mut self) {
                self.base.outputs = $func(self);
            }
        }
    };
}

component!(Pipe, map: (String, String); |component: &mut Pipe| {
    let ref inputs = component.base.inputs;
    let (from, to) = component.map.clone();
    let mut outputs = HashMap::<String, Arc<af::Array>>::new();
    match inputs.get(&from) {
        Some(x) => outputs.insert(to, x.clone()),
        None    => panic!("Input {} does not exist.", from),
    };
    outputs
});


#[test]
fn constant() {
    af::set_backend(af::Backend::CPU);

    let n_rows: u64 = 5;
    let n_cols: u64 = 3;

    let dims = af::Dim4::new(&[n_rows, n_cols, 1, 1]);
    let ones = af::constant(1.0, dims);

    let mut c0 = Pipe::new(("in".to_string(), "out".to_string()));

    c0.make_in_port("in".to_string(), dims);
    c0.make_out_port("out".to_string(), dims);

    c0.get_in_port("in".to_string()).write(Arc::new(ones));
    c0.input();
    c0.fire();
    c0.output();

    let a0 = c0.get_out_port("out".to_string()).read();
    let (r0, _) = af::sum_all(&a0);

    assert_eq!(r0, 15.0);
}
