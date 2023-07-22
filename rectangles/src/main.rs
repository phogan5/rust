#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
<<<<<<< HEAD
=======
    fn width(&self) -> bool{
        self.width > 0
    }
>>>>>>> 7a6eecb (structs and impls in rectangles project)
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
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

<<<<<<< HEAD
    println!(
        "The area of the rect1 is {} square pixels.",
        rect1.area()
    );
    println!(
        "The area of the rect2 is {} square pixels.",
        rect2.area()
    );
    println!(
        "The area of the rect3 is {} square pixels.",
        rect3.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    
    
    
    //println! ("rect1: {:#?}", rect1);
=======
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    //println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
>>>>>>> 7a6eecb (structs and impls in rectangles project)
}
