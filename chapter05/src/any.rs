use std::any::Any;
use traitobjs::{Forward, Turn, Stop, PrintableDirection};

pub struct DoesNotHaveAnyTrait<'a> {
    pub name: &'a str,
    pub count: i32,
}

pub struct DoesHaveAnyTrait {
    pub name: String,
    pub count: i32,
}

pub fn run() {
    let okay = String::from("okay");
    //let wrong = String::from("wrong");

    let directions: [&dyn Any; 7] = [
        &Forward{ blocks: 5 },
        &Turn{ slight: true, right: false },
        &Forward{ blocks: 1 },
        &Turn{ slight: false, right: true },
        &Forward{ blocks: 2 },
        &Stop{},
        //&DoesNotHaveAnyTrait{ name: wrong.as_str(), count: 16},
        &DoesHaveAnyTrait{ name: okay, count: 16},
    ];

    println!("Any-style driving directions using is:");

    for step in directions.iter() {
        if step.is::<Forward>() {
            println!("Go forward");
        }
        else if step.is::<Turn>() {
            println!("Turn here");
        }
        else if step.is::<Stop>() {
            println!("Stop now");
        }
    }

    println!("");

    println!("Any-style driving directions using downcast_ref:");

    for step in directions.iter() {
        if let Some(x) = step.downcast_ref::<Forward>() {
            x.forward();
        }
        else if let Some(x) = step.downcast_ref::<Turn>() {
            x.forward();
        }
        else if let Some(x) = step.downcast_ref::<Stop>() {
            x.forward();
        }
    }

    println!("");
}
