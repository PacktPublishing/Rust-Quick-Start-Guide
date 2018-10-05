pub fn run() {
    let mut vector = Vec::new();

    vector.push(1.5);
    //vector.push("nope");

    let x: f64 = 99.9;
    vector.push(x);

    let mut second_vector = Vec::new();
    second_vector.push("This");
    second_vector.push("works");
    second_vector.push("fine");

    println!("{} {} {}.", second_vector[0], second_vector[1], second_vector[2]);

    let third_vector = vec!["This", "works", "too"];

    println!("{} {} {}.", third_vector[0], third_vector[1], third_vector[2]);
}
