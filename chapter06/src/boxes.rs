use std::any::Any;

pub struct Person {
    pub name: String,
    pub validated: bool,
}

pub struct TreeNode {
    pub value: i32,
    pub left: Box<TreeNode>,
    pub right: Box<TreeNode>,
}

pub fn run() {
    let jack = Box::new(Person { name: "Jack".to_string(), validated: true });
    // let x = *jack; // Move the value out of jack and into x
    let x = &jack.name;
    println!("The person in the box is {}", x);

    let jill: Box<dyn Any> = Box::new(Person { name: "Jill".to_string(), validated: false });
    // println!("{}", jill.name); // Does not work, because name is not part of the Any interface

    let real_jill = jill.downcast::<Person>().unwrap();
    println!("{}", real_jill.name);
}
