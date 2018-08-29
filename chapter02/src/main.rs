pub mod expressions;

pub mod module_b;

pub mod structs_and_behavior;

pub mod module_a {
    pub fn a_thing() {
        println!("This is a thing");
    }

    pub fn a_second_thing() {
        a_thing();
        println!("This is another thing");
    }
}

fn main() -> Result<(), &'static str> {
    use module_a::a_second_thing;

    a_second_thing();

    module_b::a_second_thing();

    println!();

    expressions::operators();

    println!();

    expressions::arrays_and_tuples();

    println!();

    expressions::blocks();

    println!();

    expressions::branches();

    println!();

    expressions::loops();

    println!();

    expressions::variables();

    println!();

    structs_and_behavior::do_constrained()?;

    println!();

    Ok(())
}
