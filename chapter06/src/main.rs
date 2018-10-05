extern crate rand;

pub mod boxes;
pub mod strings;
pub mod vectors;
pub mod rc;
pub mod cell_and_refcell;
pub mod threaded;

fn main() {
    boxes::run();
    strings::run();
    vectors::run();
    rc::run();
    cell_and_refcell::run().unwrap();
    threaded::run();
}
