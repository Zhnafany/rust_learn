#[derive(Debug)] //trait for print struct as object
struct Rectangle {
    width: u32,
    height: u32,
}

//method declaration
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //assosiated function(static in C++) Rectangle::squre(u32) - to call, also as constructor
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

////////////////////
//multiple impl for one struct is valid, but to put each method separatatly
//impl Rectangle {
//    fn can_hold(&self, other: &Rectangle) -> bool {
//        self.width > other.width && self.height > other.height
//    }
//}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    //turple
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_turple(rect1)
    );

    ///////////////////////////////
    //create instanse of Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_rect(&rect1)
    );

    println!("rect1 is {:?}", rect1); // {:?} print struct to debug in one line
    println!("rect1 is {:#?}", rect1); // {:#?} print struct in format way
    //call metod
    println!("rect1 is {:#?}", rect1.area()); // {:#?} print struct in format way

    /////////////////////////////////////////
    //using metods
    #[derive(Debug,Copy,Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn distance(&self, other: &Point) -> f64 {
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);

            f64::sqrt(x_squared + y_squared)
        }
    }

    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };
    //same, rust put self, &self or &mut selt by method definition
    p1.distance(&p2);
    (&p1).distance(&p2);


    ///////////////////////////////////////////////////
    //example of method with comparison of two instanses of one struct
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

fn area_rect(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area_turple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
