use crate::xna::csharp::Rectangle;

impl Rectangle {
    pub fn from_ltrb(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Rectangle{
            x: left,
            y: top,
            width: right - left,
            height: bottom - top
        }
    }
}