pub fn operators() {
    println!("2 + 3 \t\t {}", 2 + 3);
    println!("2 * 3 \t\t {}", 2 * 3);
    println!("2 - 3 \t\t {}", 2 - 3);
    println!("2 / 3 \t\t {}", 2 / 3);
    println!("2.0 / 3.0 \t {}", 2.0 / 3.0);
    println!("2 % 3 \t\t {}", 2 % 3);
    println!("4 % 3 \t\t {}", 4 % 3);
    println!("6 & 3 \t\t {}", 6 & 3);
    println!("6 | 3 \t\t {}", 6 | 3);
    println!("1 << 3 \t\t {}", 1 << 3);
    println!("8 >> 2 \t\t {}", 8 >> 2);

    println!();
    println!("true & false \t {}", true & false);
    println!("true && false \t {}", true && false);
    println!("true | false \t {}", true | false);
    println!("true || false \t {}", true || false);
    println!("true ^ false \t {}", true ^ false);
    println!("true ^ true \t {}", true ^ true);
    println!("!true \t\t {}", !true);
    println!("!false \t\t {}", !false);

    println!();
    println!("2 > 3 \t\t {}", 2 > 3);
    println!("2 < 3 \t\t {}", 2 < 3);
    println!("2 == 3 \t\t {}", 2 == 3);
    println!("2 >= 3 \t\t {}", 2 >= 3);
    println!("2 <= 3 \t\t {}", 2 <= 3);
    println!("2 != 3 \t\t {}", 2 != 3);

    println!();
    println!("3 > 3 \t\t {}", 3 > 3);
    println!("3 < 3 \t\t {}", 3 < 3);
    println!("3 == 3 \t\t {}", 3 == 3);
    println!("3 >= 3 \t\t {}", 3 >= 3);
    println!("3 <= 3 \t\t {}", 3 <= 3);
    println!("3 != 3 \t\t {}", 3 != 3);
}

pub fn arrays_and_tuples() {
    let an_array = [1, 3, 5];
    println!("an_array \t{:?}", an_array);
    println!("an_array[1] \t{:?}", an_array[1]);
    println!("[2; 3] \t\t{:?}", [2; 3]);

    let a_tuple = (1, "wow", true);
    println!();
    println!("a_tuple \t{:?}", a_tuple);
    println!("a_tuple.1 \t{:?}", a_tuple.1);
}

pub fn blocks() {
    println!("Block result {:?}", { 2 + 2; 19 % 3; println!("In a block"); true});
}

pub fn branches() {
    if 3 > 4 {
        println!("Uh-oh. Three is greater than four.");
    }
    else if 3 == 4 {
        println!("There seems to be something wrong with math.");
    }
    else {
        println!("Three is not greater than or equal to four.");
    };

    if "hello" == "hello" {
        println!("hello equals hello");
    };

    if "hello" == "goodbye" {
        println!("hello equals goodbye");
    }
    else {
        println!("hello does not equal goodbye");
    };
}

pub fn loops() {
    let mut i = 0;
    while i < 3 {
        i = i + 1;
        println!("while loop {}", i);
    }

    println!();

    for num in 3..7 {
        println!("for loop {}", num);
    }

    println!();

    for word in ["Hello", "world", "of", "loops"].iter() {
        println!("{}", word);
    }
}

pub fn variables() {
    let x = 10;

    println!("x is {}", x);
    println!("x + 5 is {}", x + 5);

    let x: i32 = 99;
    println!("x: i32 is {}", x);

    let x: f64 = 999.0;
    println!("x: f64 is {}", x);

    //// This would be an error, because x has not had a value stored in it
    //let x: u16;
    //println!("x: u16 is {}", x);

    println!();

    let mut x = 17;
    println!("Before assigning a new value, x contains {}", x);
    x = 0;
    println!("After assigning a new value, x contains {}", x);

    for i in 0..5 {
        x = x + i;
    }
    println!("The sum of the numbers 0..5 is {}", x);
}
