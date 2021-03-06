fn main() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn find_area(&self) -> u32 {
            self.width * self.height
        }

        fn can_fit(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        fn can_fit_rotated(&self, other: &Rectangle) -> bool {
            if self.width > other.width && self.height > other.height {
                true
            } else if self.width > other.height && self.height > other.width {
                true
            } else {
                false
            }
        }
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };

    // let rect3 = Rectangle { width: 10, height: 5};

    println!("{}", rect1.find_area());
    println!("{}", rect1.can_fit(&rect2));
    println!("{}", rect1.can_fit_rotated(&rect2))
}
