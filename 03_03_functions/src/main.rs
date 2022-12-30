fn main() {
    println!("Hello, world!");
    another_function();
    print_value(10);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");
    let f = five();
    println!("{f}")
}

fn another_function() {
    println!("another function");
}

fn print_value(n: i32) {
    println!("The value of n is {n}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn five() -> i32 {
    5
}
