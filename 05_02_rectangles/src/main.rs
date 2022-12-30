fn main() {
    println!("Main v1: using separate values for width and height");
    main_v1();

    println!("Main v2: using a tuple to associate the width and height");
    main_v2();

    println!("Main v3: using a struct to represent a rectangle");
    main_v3();

    println!("Adding methods to Rectangle: can_hold");
    can_hold();
}

fn main_v1() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area  of the rectangle is {} square pixels.",
        area_v1(width1, height1)
    );
}

fn area_v1(width: u32, height: u32) -> u32 {
    width * height
}

fn main_v2() {
    let rect1 = (30, 50);
    println!(
        "The are of the rectangle is {} square pixels.",
        area_v2(rect1),
    )
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main_v3() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_v3(&rect1),
    );

    println!("rect1 is {:#?}", rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let square1 = Rectangle::square(10);
    println!("square1={square1:?}");
}

fn area_v3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn can_hold() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
