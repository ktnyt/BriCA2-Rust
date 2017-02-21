use std::sync::{Arc, Mutex};
use af;

#[derive(Clone)]
pub struct Port {
    dims: af::Dim4,
    mutex: Arc<Mutex<Arc<af::Array>>>,
}

impl Port {
    pub fn new(dims: af::Dim4) -> Port {
        Port {
            dims: dims,
            mutex: Arc::new(Mutex::new(Arc::new(af::constant(0.0, dims))))
        }
    }

    pub fn entangle(&mut self, port: &Port) {
        if self.dims != port.dims {
            panic!("Mismatched port dimensions (expected: {} actual: {})", self.dims, port.dims);
        }
        self.mutex = port.mutex.clone();
    }

    pub fn write(&mut self, value: Arc<af::Array>) {
        let mut buffer = self.mutex.lock().unwrap();
        *buffer = value.clone();
    }

    pub fn read(&self) -> Arc<af::Array> {
        self.mutex.lock().unwrap().clone()
    }
}

#[test]
fn port_works() {
    af::set_backend(af::Backend::CPU);

    let n_rows: u64 = 5;
    let n_cols: u64 = 3;

    let dims = af::Dim4::new(&[n_rows, n_cols, 1, 1]);
    let ones = af::constant(1.0, dims);

    let mut p0 = Port::new(dims);
    let p1 = Port::new(dims);

    p0.entangle(&p1);

    let a0 = p0.read();
    let a1 = p1.read();

    let (r0, _) = af::sum_all(&a0);
    let (r1, _) = af::sum_all(&a1);

    assert_eq!(r0, 0.0);
    assert_eq!(r1, 0.0);

    p0.write(Arc::new(ones));

    let a0 = p0.read();
    let a1 = p1.read();

    let (r0, _) = af::sum_all(&a0);
    let (r1, _) = af::sum_all(&a1);

    assert_eq!(r0, 15.0);
    assert_eq!(r1, 15.0);
}
