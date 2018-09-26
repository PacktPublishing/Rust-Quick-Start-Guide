pub trait PrintableDirection {
    fn forward(&self);
    fn reverse(&self);
}

pub struct Forward {
    pub blocks: u8,
}

pub struct Turn {
    pub slight: bool,
    pub right: bool,
}

pub struct Stop {}

impl PrintableDirection for Forward {
    fn forward(&self) {
        println!("Go forward {} blocks", self.blocks);
    }

    fn reverse(&self) {
        println!("Go forward {} blocks", self.blocks);
    }
}

impl PrintableDirection for Turn {
    fn forward(&self) {
        println!("Turn {}{}",
                 if self.slight {"slightly "} else {""},
                 if self.right {"right"} else {"left"});
    }

    fn reverse(&self) {
        println!("Turn {}{}",
                 if self.slight {"slightly "} else {""},
                 if self.right {"left"} else {"right"});
    }
}

impl PrintableDirection for Stop {
    fn forward(&self) {
        println!("You have reached your destination");
    }

    fn reverse(&self) {
        println!("Turn 180 degrees");
    }
}

pub fn run() {
    let mut directions: [&dyn PrintableDirection; 6] = [
        &Forward{ blocks: 5 },
        &Turn{ slight: true, right: false },
        &Forward{ blocks: 1 },
        &Turn{ slight: false, right: true },
        &Forward{ blocks: 2 },
        &Stop{},
    ];

    println!("Trait object-style driving directions:");

    for step in directions.iter() {
        step.forward();
    };

    directions.reverse();

    for step in directions.iter() {
        step.reverse();
    };

    println!("");
}
