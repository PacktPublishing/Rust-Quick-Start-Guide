pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    pub fn transpose(self) -> Point2D {
        return Point2D {x: self.y, y: self.x};
    }

    pub fn magnitude(&self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }

    pub fn unit(&mut self) {
        let mag = self.magnitude();
        self.x = self.x / mag;
        self.y = self.y / mag;
    }
}

pub fn receive_ownership(point: Point2D) -> Point2D {
    println!("Point2D{{x: {}, y: {}}} is now owned by a new scope", point.x, point.y);
    return point;
}

// pub fn uncertain_ownership(switch: bool) {
//     let point = Point2D {x: 3.0, y: 3.0};

//     if switch {
//         receive_ownership(point);
//     }

//     println!("point is Point2D{{x: {}, y: {}}}", point.x, point.y);
// }

pub fn copied_ownership(switch: bool) {
    let local = 4.0;

    if switch {
        receive_ownership(Point2D {x: local, y: 4.0});
    }

    println!("local is {}", local);
}

pub fn borrow_ownership(point: &Point2D) {
    println!("Point2D{{x: {}, y: {}}} is now borrowed by a new scope", point.x, point.y);
}

pub fn borrow_ownership_mutably(point: &mut Point2D) {
    println!("Point2D{{x: {}, y: {}}} is now borrowed by a new scope", point.x, point.y);
    point.x = 13.5;
    println!("Borrowed value changed to Point2D{{x: {}, y: {}}}", point.x, point.y);
}

pub fn set_to_six(value: &mut u32) {
    *value = 6;
}

pub fn smaller_x<'a>(value1: &'a Point2D, value2: &'a Point2D) -> &'a f64 {
    if value1.x < value2.x {
        &value1.x
    }
    else {
        &value2.x
    }
}

fn main() {
    println!("main scope just began");

    let main_1 = Point2D {x: 10.0, y: 10.0};
    let main_2 = Point2D {x: 25.0, y: 25.0};

    let main_3 = receive_ownership(main_1);
    println!("main_3 is Point2D{{x: {}, y: {}}}", main_3.x, main_3.y);

    // The value was moved, so we can't use it here anymore.
    //receive_ownership(main_1);

    let mut main_4 = main_2;
    // main_2 has been moved to main_4, so it can't be used anymore either.
    //receive_ownership(main_2);

    // The uncertain_ownership function would trigger a compiler error
    //uncertain_ownership(false);

    // The copied_ownership function is structured almost exactly the
    // same as uncertain_ownership, but does not trigger a compiler
    // error because floating point numbers have the Copy trait
    copied_ownership(false);

    borrow_ownership(&main_3);
    println!("main_3 is still here, and contains Point2D{{x: {}, y: {}}}", main_3.x, main_3.y);

    borrow_ownership_mutably(&mut main_4);
    println!("After mutable borrow, main_4 is Point2D{{x: {}, y: {}}}", main_4.x, main_4.y);

    let mut value_to_borrow = 13000;
    set_to_six(&mut value_to_borrow);
    println!("After set_to_six, the value is {}", value_to_borrow);

    println!("The smaller x is {}", smaller_x(&main_3, &main_4));

    // This is invalid, because our lifetime annotations on the
    // smaller_x function tell Rust that the value stored in smaller
    // might not be valid long enough to make it to the println command

    // let smaller;
    // {
    //     let main_5 = Point2D {x: 50.0, y: 50.0};
    //     smaller = smaller_x(&main_4, &main_5);
    // }
    // println!("The smaller x is {}", smaller);

    let main_6 = Point2D {x: 13.0, y: 13.0};
    println!("main_6 contains Point2D{{x: {}, y: {}}}", main_6.x, main_6.y);

    // main_6 no longer exists once this call is finished. The value
    // of main_6 was moved into the transpose function's scope, and
    // not returned. Instead, it returned a new Point2D, which we've
    // stored in main_7
    let mut main_7 = main_6.transpose();
    //println!("main_6 contains Point2D{{x: {}, y: {}}}", main_6.x, main_6.y);
    println!("main_7 contains Point2D{{x: {}, y: {}}}", main_7.x, main_7.y);

    // The value of main_7 is immutably borrowed into the scope of the
    // magnitude function, so calling that function has no effect on
    // the variable at all.
    println!("The magnitude of main_7 is {}", main_7.magnitude());
    println!("The magnitude of main_7 is still {}", main_7.magnitude());

    // The value of main_7 is borrowed mutably into the scope of the
    // unit function, and changed
    main_7.unit();
    println!("After calling unit(), main_7 contains Point2D{{x: {}, y: {}}}", main_7.x, main_7.y);
    println!("After calling unit(), the magnitude of main_7 is {}", main_7.magnitude());

    println!("main scope is about to end");
}
