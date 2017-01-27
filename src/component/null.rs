use port::Port;
use unit::*;
use component::*;
use std::collections::HashMap;
use std::sync::Arc;
use af;

#[derive(Clone)]
pub struct Null {
    base: ComponentStruct,
}

impl Null {
    pub fn new() -> Self {
        Null {
            base: ComponentStruct::new(),
        }
    }
}

impl Unit for Null {
    delegate! {
        for base;
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
}

impl Component for Null {
    delegate! {
        for base;
        fn input(&mut self);
        fn output(&mut self);
        fn get_input(&mut self, key: &str) -> Arc<af::Array>;
        fn get_output(&mut self, key: &str) -> Arc<af::Array>;
    }

    fn fire(&mut self) {}
}

#[test]
fn it_works() {
    af::set_backend(af::Backend::CPU);

    let n_rows: u64 = 5;
    let n_cols: u64 = 3;

    let dims = af::Dim4::new(&[n_rows, n_cols, 1, 1]);

    let mut c0 = Null::new();

    c0.make_out_port("in", dims);
}
