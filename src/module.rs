use port::Port;
use unit::*;
use component::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use af;

#[derive(Clone)]
pub struct Module {
    unit: UnitStruct,
    components: HashMap<String, Arc<Mutex<Arc<Component>>>>,
    submodules: HashMap<String, Arc<Mutex<Arc<Module>>>>,
}

impl Module {
    pub fn new() -> Self {
        Module {
            unit: UnitStruct::new(),
            components: HashMap::new(),
            submodules: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, key: &str, component: Arc<Component>) {
        self.components.insert(key.to_string(), Arc::new(Mutex::new(component)));
    }

    pub fn get_component(&mut self, key: &str) -> Arc<Mutex<Arc<Component>>> {
        match self.components.get_mut(&key.to_string()) {
            Some(x) => x.clone(),
            None    => panic!("Component `{}` does not exist", key),
        }
    }

    pub fn add_submodule(&mut self, key: &str, submodule: Arc<Module>) {
        self.submodules.insert(key.to_string(), Arc::new(Mutex::new(submodule)));
    }

    pub fn get_submodule(&mut self, key: &str) -> Arc<Mutex<Arc<Module>>> {
        match self.submodules.get_mut(&key.to_string()) {
            Some(x) => x.clone(),
            None    => panic!("Component `{}` does not exist", key),
        }
    }
}

impl Unit for Module {
    delegate! {
        for unit;
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

#[test]
fn module_works() {
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

    let mut m0 = Module::new();

    m0.add_component("c0", Arc::new(c0));
    m0.add_component("c1", Arc::new(c1));
    m0.add_component("c2", Arc::new(c2));

    {
        let mutex0 = m0.get_component("c0");
        let mutex1 = m0.get_component("c1");
        let mutex2 = m0.get_component("c2");
        let ref mut arc0 = *mutex0.lock().unwrap();
        let ref mut arc1 = *mutex1.lock().unwrap();
        let ref mut arc2 = *mutex2.lock().unwrap();
        let ref mut c0 = Arc::get_mut(arc0).unwrap();
        let ref mut c1 = Arc::get_mut(arc1).unwrap();
        let ref mut c2 = Arc::get_mut(arc2).unwrap();

        let a0 = c0.get_out_port("out").unwrap().read();
        let a1 = c1.get_in_port("in").unwrap().read();
        let a2 = c1.get_out_port("out").unwrap().read();
        let a3 = c2.get_in_port("in").unwrap().read();
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
    }

    {
        let mutex0 = m0.get_component("c0");
        let mutex1 = m0.get_component("c1");
        let mutex2 = m0.get_component("c2");
        let ref mut arc0 = *mutex0.lock().unwrap();
        let ref mut arc1 = *mutex1.lock().unwrap();
        let ref mut arc2 = *mutex2.lock().unwrap();
        let ref mut c0 = Arc::get_mut(arc0).unwrap();
        let ref mut c1 = Arc::get_mut(arc1).unwrap();
        let ref mut c2 = Arc::get_mut(arc2).unwrap();

        c0.input();
        c1.input();
        c2.input();
        c0.fire();
        c1.fire();
        c2.fire();
        c0.output();
        c1.output();
        c2.output();
    }

    {
        let mutex0 = m0.get_component("c0");
        let mutex1 = m0.get_component("c1");
        let mutex2 = m0.get_component("c2");
        let ref mut arc0 = *mutex0.lock().unwrap();
        let ref mut arc1 = *mutex1.lock().unwrap();
        let ref mut arc2 = *mutex2.lock().unwrap();
        let ref mut c0 = Arc::get_mut(arc0).unwrap();
        let ref mut c1 = Arc::get_mut(arc1).unwrap();
        let ref mut c2 = Arc::get_mut(arc2).unwrap();

        let a0 = c0.get_out_port("out").unwrap().read();
        let a1 = c1.get_in_port("in").unwrap().read();
        let a2 = c1.get_out_port("out").unwrap().read();
        let a3 = c2.get_in_port("in").unwrap().read();
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
    }

    {
        let mutex0 = m0.get_component("c0");
        let mutex1 = m0.get_component("c1");
        let mutex2 = m0.get_component("c2");
        let ref mut arc0 = *mutex0.lock().unwrap();
        let ref mut arc1 = *mutex1.lock().unwrap();
        let ref mut arc2 = *mutex2.lock().unwrap();
        let ref mut c0 = Arc::get_mut(arc0).unwrap();
        let ref mut c1 = Arc::get_mut(arc1).unwrap();
        let ref mut c2 = Arc::get_mut(arc2).unwrap();

        c0.input();
        c1.input();
        c2.input();
        c0.fire();
        c1.fire();
        c2.fire();
        c0.output();
        c1.output();
        c2.output();
    }

    {
        let mutex0 = m0.get_component("c0");
        let mutex1 = m0.get_component("c1");
        let mutex2 = m0.get_component("c2");
        let ref mut arc0 = *mutex0.lock().unwrap();
        let ref mut arc1 = *mutex1.lock().unwrap();
        let ref mut arc2 = *mutex2.lock().unwrap();
        let ref mut c0 = Arc::get_mut(arc0).unwrap();
        let ref mut c1 = Arc::get_mut(arc1).unwrap();
        let ref mut c2 = Arc::get_mut(arc2).unwrap();

        let a0 = c0.get_out_port("out").unwrap().read();
        let a1 = c1.get_in_port("in").unwrap().read();
        let a2 = c1.get_out_port("out").unwrap().read();
        let a3 = c2.get_in_port("in").unwrap().read();
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
    }

    {
        let mutex0 = m0.get_component("c0");
        let mutex1 = m0.get_component("c1");
        let mutex2 = m0.get_component("c2");
        let ref mut arc0 = *mutex0.lock().unwrap();
        let ref mut arc1 = *mutex1.lock().unwrap();
        let ref mut arc2 = *mutex2.lock().unwrap();
        let ref mut c0 = Arc::get_mut(arc0).unwrap();
        let ref mut c1 = Arc::get_mut(arc1).unwrap();
        let ref mut c2 = Arc::get_mut(arc2).unwrap();

        c0.input();
        c1.input();
        c2.input();
        c0.fire();
        c1.fire();
        c2.fire();
        c0.output();
        c1.output();
        c2.output();
    }

    {
        let mutex0 = m0.get_component("c0");
        let mutex1 = m0.get_component("c1");
        let mutex2 = m0.get_component("c2");
        let ref mut arc0 = *mutex0.lock().unwrap();
        let ref mut arc1 = *mutex1.lock().unwrap();
        let ref mut arc2 = *mutex2.lock().unwrap();
        let ref mut c0 = Arc::get_mut(arc0).unwrap();
        let ref mut c1 = Arc::get_mut(arc1).unwrap();
        let ref mut c2 = Arc::get_mut(arc2).unwrap();

        let a0 = c0.get_out_port("out").unwrap().read();
        let a1 = c1.get_in_port("in").unwrap().read();
        let a2 = c1.get_out_port("out").unwrap().read();
        let a3 = c2.get_in_port("in").unwrap().read();
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
}
