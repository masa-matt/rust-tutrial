fn main() {
    methods();
    methods_with_params();
    associated_functions();
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

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }
}

// it can be separated
impl Rectangle {
    // this is associated function, not method. because they don't need instance.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn methods() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // this is same
    // p1.distance(&p2);
    // (&p1).distance(&p2);
}

fn methods_with_params() {
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

fn associated_functions() {
    let rect1 = Rectangle::square(50);

    println!("The square is {:?}", rect1);
}
