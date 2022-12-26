fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // fails:
    // if number {
    //    ^^^^^^ expected `bool`, found integer
    //     println!("ooops")
    // }
    if number != 0 {
        println!("number was something other than zero.")
    }

    foo_bar_baz(5);
    foo_bar_baz(7);
    foo_bar_baz(35);

    // loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!");

    // loop over collection

    // works but it's error prone
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // loop over the container with for in
    for element in a {
        println!("the value is: {element}")
    }

    // liftoff countdown using for in range
    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("LIFTOFF!!!")
}

fn foo_bar_baz(n: u32) {
    // For every multiple of 5 print "Foo", for every multiple
    // of 7 print "Bar" and for every multiple of both 5 and 7
    // print "FooBar" instead of the number.
    if n % 5 == 0 {
        print!("Foo");
    }
    if n % 7 == 0 {
        print!("Bar");
    }
    print!("\n")
}
