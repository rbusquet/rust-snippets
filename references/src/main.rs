fn main() {
    let s1 = String::from("Hello, world!");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // change_wrong(&s1); // errors
    let mut s2 = String::from("hello");
    change_ok(&mut s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_wrong(s: &String) {
    // s.push_str("AHAAAS");
}

fn change_ok(s: &mut String) {
    s.push_str("that's ok");
}
