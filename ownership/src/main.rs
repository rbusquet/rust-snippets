fn main() {
    {
        // string literals are hardcoded in the binary,
        let s = "hello"; // scope is the "owner"

        // string literals are immutable:
        // s += "world";
        // -^^^^^^^^^^^
        // |
        // cannot use `+=` on type `&str`

        println!("{s}");
    }
    // println!("{s}")
    //            ^ not found in this scope
    {
        // String allocates memory in the heap since they
        // can grow at runtime.
        let mut s = String::from("hello"); // s is valid
        s.push_str(", world!");
        println!("{}", s)
    }

    {
        let x = 5; // assign 5 to x
        let y = x; // copy x to y
        println!("x={x} y={y}");

        let s1 = String::from("hello");
        // s1 is a data structure containing a pointer to the string value in the heap
        // plus length and capacity information.
        let _s2 = s1; // this copies the data structure, effectively,
                      // the pointer to the actual string

        // println!("{}", s1); // oh wow this is an error, s1 is invalid after s2 = s1
        //                     // s2 "borrows" from s1
        // the following works just fine
        let a = {
            let mut b = String::from("hello");
            b.push_str(" world");
            b
        };
        println!("{a}");

        // but what if I DO want to deep copy the values? Cloning!
        {
            let s1 = String::from("hello");
            let s2 = s1.clone();
            // from the book:
            // > It’s a visual indicator that something different is going on.
            println!("s1={}, s2={}", s1, s2);
        }

        // ownership and functions
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here

        // println!("{s}"); // this doesn't work anymore

        let x = 5; // x comes into scope

        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
        println!("{}", x);
    }
    {
        let s1 = gives_ownership();
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        println!("{} {}", s1, s3 /*, s2 here breaks */);
    } // s1 is dropped, s2 was moved, nothing happens, s3 is dropped

    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }

    // using a value without moving ownership: references
    {
        // example of rejected codes:
        // 1. the compiler checks both arms of the condition:
        // let x = if true { 1 } else { "hello" };
        // assert_eq!(x + 1, 2);
        // 2. s2 is not used in move_two, but the compiler consideres the value
        //    moved
        // fn move_two(s1: String, s2: String) -> String {
        //     s1
        // }
        // let (s1, s2) = (String::from("a"), String::from("b"));
        // let s3 = move_two(s1, s2);
        // println!("{} {}", s2, s3);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}
