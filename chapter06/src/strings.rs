pub fn run() {
    let mut text = String::new();
    text.push('W');
    text.push_str("elcome to mutable strings");
    text.insert_str(11, "exciting ");
    text.replace_range(28.., "text");
    println!("{}", text);
}
