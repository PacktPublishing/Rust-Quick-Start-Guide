pub struct DemoStruct {
    pub id: u64,
    pub name: String,
    pub probability: f64,
}

pub fn might_fail(id: u64) -> Result<DemoStruct, &'static str> {
    if id % 2 == 0 {
        Ok(DemoStruct { id: id, name: String::from("An Even Thing"), probability: 0.2})
    }
    else {
        Err("Only even numbers are allowed")
    }
}

pub fn borrow_demostruct(x: &DemoStruct) {
    println!("Borrowed {}", x.name);
}

fn main() {
    let source1 = DemoStruct { id: 31, name: String::from("Example Thing"), probability: 0.42 };

    // This will cause the program to refuse to compile, because 0.99
    // is only one of many possible values, and a let expression needs
    // to work in all cases.
    //let DemoStruct { id: 31, name: y, probability: z } = source1;

    let DemoStruct { id: x, name: y, probability: z } = source1;

    println!("id: {}, name: '{}', probability: {}", x, y, z);

    let source2 = DemoStruct { id: 35, name: String::from("Another Thing"), probability: 0.42 };
    let source3 = DemoStruct { id: 63, name: String::from("Super Thing"), probability: 0.99 };

    if let DemoStruct { id: 63, name: y, probability: z } = source2 {
        println!("When id is 63, name is {} and probability is {}", y, z);
    }

    if let DemoStruct { id: 63, name: y, probability: z } = source3 {
        println!("When id is 63, name is {} and probability is {}", y, z);
    }

    let source4 = DemoStruct { id: 36, name: String::from("Another Thing"), probability: 0.42 };

    if false {
        println!("This never happens");
    }
    else if let DemoStruct{ id: 35, name: y, probability: z } = source4 {
        println!("When id is 35, name is {} and probability is {}", y, z);
    }
    else if let DemoStruct{ id: 36, name: y, probability: z } = source4 {
        println!("When id is 36, name is {} and probability is {}", y, z);
    }
    else {
        println!("None of the conditions matched");
    }

    if let Ok(x) = might_fail(37) {
        println!("Odd succeeded, name is {}", x.name);
    }

    if let Ok(x) = might_fail(38) {
        println!("Even succeeded, name is {}", x.name);
    }

    match might_fail(39) {
        Ok(x) => { println!("Odd succeeded, name is {}", x.name) }
        Err(x) => { println!("Odd failed, message is '{}'", x) }
    }

    match might_fail(39) {
        Ok(x) => { println!("Odd succeeded, name is {}", x.name) }
        Err(_) => { println!("Odd failed! Woe is me.") }
    }

    match might_fail(39) {
        Ok(x) => { println!("Odd succeeded, name is {}", x.name) }
        _ => { println!("If none of the above patterns match, _ certainly will") }
    }

    //The source5.name value is moved into a different variable
    //during the if let, so this will cause a compiler error
    // let source5 = DemoStruct { id: 40, name: String::from("A Surprising Thing"), probability: 0.93 };

    // if let DemoStruct {id: 41, name: x, probability: _} = source5 {
    //     println!("Extracted name: {}", x);
    // }

    // println!("source5.name is {}", source5.name);

    //Trying to borrow x by putting an & in front of it doesn't work,
    //because that is just saying that we expect the value we're
    //matching to be a reference itself
    // let source5 = DemoStruct { id: 40, name: String::from("A Surprising Thing"), probability: 0.93 };

    // if let DemoStruct {id: 41, name: &x, probability: _} = source5 {
    //     println!("Extracted name: {}", x);
    // }

    // println!("source5.name is {}", source5.name);

    let source5 = DemoStruct { id: 40, name: String::from("A Surprising Thing"), probability: 0.93 };

    if let DemoStruct {id: 41, name: ref x, probability: _} = source5 {
        println!("Extracted name: {}", x);
    }

    println!("source5.name is {}", source5.name);

    let ref borrowed1 = source5;
    let borrowed2 = &source5;

    println!("borrowed1.name = {}", borrowed1.name);
    println!("borrowed2.name = {}", borrowed2.name);

    let ref borrowed_borrow = &source5;
    borrow_demostruct(borrowed_borrow);

    //Uncomment this to trigger a compiler error which will let you
    //see the data type of borrowed_borrow
    // let x: () = borrowed_borrow;

    let (_, (_, x, _, _), _) = ((5, 6, 7), (8, 9, 10, 11), (12, 13, 14, 15));
    println!("x is {}", x);

    match might_fail(38) {
        Ok(DemoStruct {id: 38, name: ref name, probability: _}) => {
            println!("Even succeeded with the proper id: name is {}", name)
        }
        Ok(DemoStruct {id: ref id, name: ref name, probability: _}) => {
            println!("Even succeeded with the wrong id: id is {}, name is {}", id, name)
        }
        Err(_) => { println!("Even failed! Woe is me.") }
    }

    if let (1, x @ (_, _), _) = (1, (2, 3), (4, 5, 6)) {
        println!("matched x to {:?}", x);
    }

    if let DemoStruct {id: 40, ..} = source5 {
        println!("id is 40, don't care about the rest");
    }

    let x = 5;

    let source6 = DemoStruct {id: 7, name: String::from("oops"), probability: 0.26};
    if let DemoStruct { id: 7, name: x, probability: _ } = source6 {
        println!("The pattern matched, x is {}", x);
    }

    let source7 = DemoStruct {id: 7, name: String::from("oops"), probability: 0.26};
    match source7 {
        DemoStruct { id: y, name: _, probability: _ } if y == x => {
            println!("The pattern with match guard matched, y is {}", y);
        }
        _ => {
            println!("The pattern with match guard did not match")
        }
    }
}
