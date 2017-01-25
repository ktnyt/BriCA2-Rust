use port::Port;
use unit::*;
use component::*;
use std::collections::HashMap;
use std::sync::Arc;
use af;

pub struct Pipe {
    base: ComponentStruct,
    map: (String, String),
}

impl Pipe {
    pub fn new(map: (String, String)) -> Self {
        Pipe {
            base: ComponentStruct::new(),
            map: map,
        }
    }
}

impl Unit for Pipe {
    delegate! {
        for base;
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
}

impl Component for Pipe {
    delegate! {
        for base;
        fn input(&mut self);
        fn output(&mut self);
    }

    fn fire(&mut self) {
        let ref inputs = self.base.inputs;
        let (from, to) = self.map.clone();
        let mut outputs = HashMap::<String, Arc<af::Array>>::new();
        match inputs.get(&from) {
            Some(x) => outputs.insert(to, x.clone()),
            None    => panic!("Input {} does not exist.", from),
        };
        self.base.outputs = outputs;
    }
}

#[test]
fn it_works() {
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
