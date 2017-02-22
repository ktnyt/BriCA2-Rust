extern crate arrayfire as af;

#[macro_use]
pub mod delegate;
pub mod port;
pub mod unit;
pub mod component;
pub mod module;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
