pub fn a_thing() {
    println!("This is a module_b thing");
}

pub fn a_second_thing() {
    a_thing();
    println!("This is another module_b thing");
}
