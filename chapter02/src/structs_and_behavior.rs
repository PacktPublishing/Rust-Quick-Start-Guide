pub struct Constrained {
    pub min: i32,
    pub max: i32,
    current: i32,
}

pub fn new_constrained(min: i32, max: i32, current: i32) -> Result<Constrained, &'static str> {
    if min <= current && max >= current {
        Ok(Constrained {min, max, current})
    }
    else {
        Err("Initial current value must be within the constraints")
    }
}

impl Constrained {
    pub fn set(&mut self, value: i32) {
        self.current = value;
    }

    pub fn get(&self) -> i32 {
        if self.current < self.min {
            return self.min;
        }
        else if self.current > self.max {
            return self.max;
        }
        else {
            return self.current;
        };
    }

    pub fn alternate_get(&self) -> i32 {
        if self.current < self.min {
            self.min
        }
        else if self.current > self.max {
            self.max
        }
        else {
            self.current
        }
    }
}


pub fn do_constrained() -> Result<(), &'static str>{
    let mut cons = new_constrained(0, 100, 0)?;

    cons.min = 5;
    cons.max = 10;

    println!("Initial constrained value is {}", cons.get());

    cons.set(7);

    println!("Intermediate constrained value is {}", cons.get());

    cons.set(99);

    println!("Final constrained value is {}", cons.get());

    println!("Now attempting to initialize an invalid Constrained value");

    let mut cons_invalid = new_constrained(100, 0, 0)?;

    Ok(())
}
