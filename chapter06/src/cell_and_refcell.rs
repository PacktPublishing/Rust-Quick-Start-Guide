use std::cell::{Cell, RefCell, BorrowMutError};

pub fn run() -> Result<(), BorrowMutError> {
    let cell = Cell::new("Let me out!".to_string());
    println!("{}", cell.replace("Don't put me in there.".to_string()));
    println!("{}", cell.replace("I didn't do anything.".to_string()));
    cell.set("You'll never hold me, copper!".to_string());
    println!("{}", cell.into_inner());
    //println!("{}", cell.replace("I still didn't do anything.".to_string()));

    let refcell = RefCell::new("It's a string".to_string());

    match refcell.try_borrow() {
        Ok(x) => { println!("Borrowed: {}", x); }
        Err(_) => { println!("Couldn't borrow first try"); }
    };

    let borrowed_mutably = refcell.try_borrow_mut()?;

    match refcell.try_borrow() {
        Ok(x) => { println!("Borrowed: {}", x); }
        Err(_) => { println!("Couldn't borrow second try"); }
    };

    println!("Mutable borrow is still alive: {}", borrowed_mutably);

    Ok(())
}
