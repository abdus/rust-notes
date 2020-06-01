#[derive(Debug)]
struct Rect {
    height: i32,
    width: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn can_hold(&self, r: &Rect) -> bool {
        self.width > r.width && self.height > r.height
    }
}

impl Rect {
    fn square(size: i32) -> Rect {
        Rect {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let r1 = Rect {
        height: 4,
        width: 5,
    };
    let r2 = Rect {
        height: 2,
        width: 3,
    };

    let square = Rect::square(10);

    println!("Area: {}", r1.area()); // 20
    println!("Can Hold r2: {}", r1.can_hold(&r2)); // true
    println!("Square: {:?}", square); // Rect { height: 10, width: 10 }
}

