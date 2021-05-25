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
            length: 13,
            width: 4,
        };
        // second argument in assert, assert_eq, and assert_ne will be a custom error message!
        assert!(
            larger.can_hold(smaller),
            "We could not fit {} into {}",
            "larger",
            "smaller"
        );
    }
    #[test]
    fn assert_hold_fail() {
        let larger = Rectangle {
            length: 12,
            width: 9,
        };
        let smaller = Rectangle {
            length: 3,
            width: 4,
        };
        // correct case is false, so we add a bang to negate
        assert!(!smaller.can_hold(larger))
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
