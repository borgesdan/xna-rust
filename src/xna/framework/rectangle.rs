use crate::xna::framework::{Point, Rectangle};

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    pub fn left(&self) -> i32 {
        return self.x;
    }

    pub fn right(&self) -> i32 {
        return self.x + self.width;
    }

    pub fn top(&self) -> i32 {
        return self.y;
    }

    pub fn bottom(&self) -> i32 {
        return self.y + self.height;
    }

    pub fn location(&self) -> Point {
        Point { x: self.x, y: self.y }
    }

    pub fn center(&self) -> Point {
        Point { x: self.x + self.width / 2, y: self.y + self.height / 2 }
    }

    pub fn empty(&self) -> bool {
        self.x == 0 && self.y == 0 && self.width == 0 && self.height == 0
    }

    pub fn offset(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    pub fn inflate(&mut self, horizontal_amount: i32, vertical_amount: i32) {
        self.x -= horizontal_amount;
        self.y -= vertical_amount;
        self.width += horizontal_amount * 2;
        self.height += vertical_amount * 2;
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.x <= x && x < self.x + self.width
            && self.y <= y && y < self.y + self.height
    }

    pub fn contains_rectangle(&self, value: &Rectangle) -> bool {
        self.x <= value.x && value.x + value.width <= self.x + self.width
            && self.y <= value.y && value.y + value.height <= self.y + self.height
    }

    pub fn intersects(&self, value: &Rectangle) -> bool {
        value.x < self.x + self.width && self.x < value.x + value.width
            && value.y < self.y + self.height && self.y < value.y + value.height
    }

    pub fn intersect(value1: &Rectangle, value2: &Rectangle) -> Rectangle {
        let num1 = value1.x + value1.width;
        let num2 = value2.x + value2.width;
        let num3 = value1.y + value1.height;
        let num4 = value2.y + value2.height;
        let num5 = if value1.x > value2.x { value1.x } else { value2.x };
        let num6 = if value1.y > value2.y { value1.y } else { value2.y };
        let num7 = if num1 < num2 { num1 } else { num2 };
        let num8 = if num3 < num4 { num3 } else { num4 };

        if num7 > num5 && num8 > num6
        {
            return Rectangle::new(num5, num6, num7 - num5, num8 - num6);
        }

        Self::new(0, 0, 0, 0)
    }

    pub fn union(value1: &Rectangle, value2: &Rectangle) -> Rectangle {
        let num1 = value1.x + value1.width;
        let num2 = value2.x + value2.width;
        let num3 = value1.y + value1.height;
        let num4 = value2.y + value2.height;
        let num5 = if value1.x < value2.x { value1.x } else { value2.x };
        let num6 = if value1.y < value2.y { value1.y } else { value2.y };
        let num7 = if num1 > num2 { num1 } else { num2 };
        let num8 = if num3 > num4 { num3 } else { num4 };

        Self::new(num5, num6, num7, num8 - num6)
    }

    pub fn equals(&self, other: &Rectangle) -> bool {
        self.x == other.x && self.y == other.y
            && self.width == other.width && self.height == other.height
    }
}