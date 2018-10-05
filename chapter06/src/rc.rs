use std::io;
use std::io::prelude::*;
use std::rc::{Rc,Weak};

pub fn make_vector_of_rcs() -> Vec<Rc<String>> {
    let ada = Rc::new("Ada".to_string());
    let mel = Rc::new("Mel".to_string());

    return vec![
        Rc::clone(&ada),
        Rc::clone(&mel),
        Rc::clone(&ada),
        Rc::clone(&ada),
        Rc::clone(&mel),
        Rc::clone(&ada),
        Rc::clone(&mel),
    ];
}

pub fn run() {
    let mut ada_and_mel = make_vector_of_rcs();

    while ada_and_mel.len() > 0 {
        println!("{:?}", ada_and_mel);

        print!("Remove which: ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let idx: usize = line.trim().parse().unwrap();
        ada_and_mel.remove(idx);
    }

}
