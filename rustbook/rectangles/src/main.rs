#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: dbg!(2 * 15),
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
    let sq = Rectangle::square(10);

    if rect1.width() {
        println!("The width is {}", rect1.width);
    }

    // println!("The area of the rectangle {:#?} is {}", rect1, area(&rect1));
    println!("The area of the rectangle {:#?} is {}", rect1, rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold sq? {}", rect1.can_hold(&sq));
}
//
// fn area(rect: &Rectangle) -> u32 {
//     rect.height * rect.width
// }
