pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
}

pub enum Drive {
    Forward(u8),
    Turn{slight: bool, right: bool},
    Stop,
}

pub fn run() {
    let _color = Color::Green;

    let directions = [
        Drive::Forward(3),
        Drive::Turn{slight: false, right: true},
        Drive::Forward(1),
        Drive::Stop,
    ];

    println!("Enumeration-style directions:");

    for step in directions.iter() {
        match step {
            Drive::Forward(blocks) => {
                println!("Drive forward {} blocks", blocks);
            },
            Drive::Turn{slight, right} => {
                println!("Turn {}{}",
                         if *slight {
                             "slightly "
                         }
                         else {
                             ""
                         },
                         if *right {
                             "right"
                         }
                         else {
                             "left"
                         }
                );
            },
            Drive::Stop => {
                println!("You have reached your destination");
            }
        };
    };

    println!("");
}
