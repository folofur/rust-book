#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn i_fail() {
        panic!("never meant to work");
    }
    #[test]
    fn assert_hold() {
        let larger = Rectangle {
            length: 12,
            width: 9,
        };
        let smaller = Rectangle {
            length: 3,
            width: 4,
        };

        assert!(larger.can_hold(smaller));
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
