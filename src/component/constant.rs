use port::Port;
use unit::*;
use component::*;
use std::collections::HashMap;
use std::sync::Arc;
use af;

#[derive(Clone)]
pub struct Constant {
    base: ComponentStruct,
    value: af::Array,
}

impl Constant {
    pub fn new(value: af::Array) -> Self {
        Constant {
            base: ComponentStruct::new(),
            value: value,
        }
    }
}

impl Unit for Constant {
    delegate! {
        for base;
        fn make_in_port(&mut self, key: &str, dims: af::Dim4);
        fn get_in_port(&mut self, key: &str) -> &mut Port;
        fn get_in_ports(&mut self) -> &mut HashMap<String, Port>;
        fn remove_in_port(&mut self, key: &str);
        fn alias_in_port(&mut self, from: &str, other: &mut Unit, to: &str);
        fn make_out_port(&mut self, key: &str, dims: af::Dim4);
        fn get_out_port(&mut self, key: &str) -> &mut Port;
        fn get_out_ports(&mut self) -> &mut HashMap<String, Port>;
        fn remove_out_port(&mut self, key: &str);
        fn alias_out_port(&mut self, from: &str, other: &mut Unit, to: &str);
        fn connect(&mut self, from: &str, other: &mut Unit, to: &str);
    }
}

impl Component for Constant {
    delegate! {
        for base;
        fn input(&mut self);
        fn output(&mut self);
        fn get_input(&mut self, key: &str) -> Arc<af::Array>;
        fn get_output(&mut self, key: &str) -> Arc<af::Array>;
    }

    fn fire(&mut self) {
        let value = self.value.clone();
        let mut outputs = HashMap::<String, Arc<af::Array>>::new();
        outputs.insert("out".to_string(), Arc::new(value));
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

    let mut c0 = Constant::new(ones);

    c0.make_out_port("out", dims);

    let a0 = c0.get_out_port("out").read();
    let (r0, _) = af::sum_all(&a0);

    assert_eq!(r0, 0.0);

    c0.input();
    c0.fire();
    c0.output();

    let a0 = c0.get_out_port("out").read();
    let (r0, _) = af::sum_all(&a0);

    assert_eq!(r0, 15.0);

}
