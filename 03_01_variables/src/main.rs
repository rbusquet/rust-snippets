fn main() {
    // let x = 5;
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}s");
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x is {x}");

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    let five_hundred = tup.0;
    println!("x={x}, y={y}, z={z}, five_hundred={five_hundred}");
    let _a = [1, 2, 3, 4, 5];
    // println!("{a}");

    let t = ([1; 2], [3; 4]);
    let (a, _) = t;

    println!("{}", a[0] + t.1[0]);
    another_function();
}

fn another_function() {
    println!("Another function!")
}
