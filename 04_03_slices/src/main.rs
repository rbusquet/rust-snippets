fn main() {
    let mut s = String::from("hello world");
    let word = first_word_v1(&s);
    println!("{}", word);
    s.clear();

    println!("{}", word); // still 5

    let mut s = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let s2: &String = &s; // not a slice, for comparison
    println!("{} {} {}", hello, world, s2);

    let word = first_word_v2(&s);
    // s.clear();  errors

    println!("{}", word);

    let word = first_word_v3(&s);
    println!("{}", word)
}

fn first_word_v1(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_v3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
