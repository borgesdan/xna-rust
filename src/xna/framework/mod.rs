use crate::xna::framework::graphics::IPackedVector;
pub mod graphics;

#[derive(Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Default)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Default)]
pub struct Color {
    packed_value: u32,
}

#[derive(Default)]
pub struct Matrix {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m14: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m24: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
    pub m34: f32,
    pub m41: f32,
    pub m42: f32,
    pub m43: f32,
    pub m44: f32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn zero() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn equals(p1: &Point, p2: &Point) -> bool {
        p1.x == p2.x && p1.y == p2.y
    }
}

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

        return Self::new(0, 0, 0, 0);
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

        return Self::new(num5, num6, num7, num8 - num6);
    }

    pub fn equals(&self, other: &Rectangle) -> bool {
        self.x == other.x && self.y == other.y
        && self.width == other.width && self.height == other.height
    }
}

impl Color {
    pub fn from_packed_value(packed_value: u32) -> Color {
        Color { packed_value }
    }

    pub fn r(&self)
             -> u8 { self.packed_value as u8 }

    pub fn g(&self)
             -> u8 { (self.packed_value >> 8) as u8 }

    pub fn b(&self)
             -> u8 { (self.packed_value >> 16) as u8 }

    pub fn a(&self)
             -> u8 { (self.packed_value >> 24) as u8 }

    pub fn Transparent() -> Color { Color::from_packed_value(0) }

    pub fn AliceBlue() -> Color { Color::from_packed_value(4294965488) }

    pub fn AntiqueWhite() -> Color { Color::from_packed_value(4292340730) }

    pub fn Aqua() -> Color { Color::from_packed_value(4294967040) }

    pub fn Aquamarine() -> Color { Color::from_packed_value(4292149119) }

    pub fn Azure() -> Color { Color::from_packed_value(4294967280) }

    pub fn Beige() -> Color { Color::from_packed_value(4292670965) }

    pub fn Bisque() -> Color { Color::from_packed_value(4291093759) }

    pub fn Black() -> Color { Color::from_packed_value(4278190080) }

    pub fn BlanchedAlmond() -> Color { Color::from_packed_value(4291685375) }

    pub fn Blue() -> Color { Color::from_packed_value(4294901760) }

    pub fn BlueViolet() -> Color { Color::from_packed_value(4293012362) }

    pub fn Brown() -> Color { Color::from_packed_value(4280953509) }

    pub fn BurlyWood() -> Color { Color::from_packed_value(4287084766) }

    pub fn CadetBlue() -> Color { Color::from_packed_value(4288716383) }

    pub fn Chartreuse() -> Color { Color::from_packed_value(4278255487) }

    pub fn Chocolate() -> Color { Color::from_packed_value(4280183250) }

    pub fn Coral() -> Color { Color::from_packed_value(4283465727) }

    pub fn CornflowerBlue() -> Color { Color::from_packed_value(4293760356) }

    pub fn Cornsilk() -> Color { Color::from_packed_value(4292671743) }

    pub fn Crimson() -> Color { Color::from_packed_value(4282127580) }

    pub fn Cyan() -> Color { Color::from_packed_value(4294967040) }

    pub fn DarkBlue() -> Color { Color::from_packed_value(4287299584) }

    pub fn DarkCyan() -> Color { Color::from_packed_value(4287335168) }

    pub fn DarkGoldenrod() -> Color { Color::from_packed_value(4278945464) }

    pub fn DarkGray() -> Color { Color::from_packed_value(4289309097) }

    pub fn DarkGreen() -> Color { Color::from_packed_value(4278215680) }

    pub fn DarkKhaki() -> Color { Color::from_packed_value(4285249469) }

    pub fn DarkMagenta() -> Color { Color::from_packed_value(4287299723) }

    pub fn DarkOliveGreen() -> Color { Color::from_packed_value(4281297749) }

    pub fn DarkOrange() -> Color { Color::from_packed_value(4278226175) }

    pub fn DarkOrchid() -> Color { Color::from_packed_value(4291572377) }

    pub fn DarkRed() -> Color { Color::from_packed_value(4278190219) }

    pub fn DarkSalmon() -> Color { Color::from_packed_value(4286224105) }

    pub fn DarkSeaGreen() -> Color { Color::from_packed_value(4287347855) }

    pub fn DarkSlateBlue() -> Color { Color::from_packed_value(4287315272) }

    pub fn DarkSlateGray() -> Color { Color::from_packed_value(4283387695) }

    pub fn DarkTurquoise() -> Color { Color::from_packed_value(4291939840) }

    pub fn DarkViolet() -> Color { Color::from_packed_value(4292018324) }

    pub fn DeepPink() -> Color { Color::from_packed_value(4287829247) }

    pub fn DeepSkyBlue() -> Color { Color::from_packed_value(4294950656) }

    pub fn DimGray() -> Color { Color::from_packed_value(4285098345) }

    pub fn DodgerBlue() -> Color { Color::from_packed_value(4294938654) }

    pub fn Firebrick() -> Color { Color::from_packed_value(4280427186) }

    pub fn FloralWhite() -> Color { Color::from_packed_value(4293982975) }

    pub fn ForestGreen() -> Color { Color::from_packed_value(4280453922) }

    pub fn Fuchsia() -> Color { Color::from_packed_value(4294902015) }

    pub fn Gainsboro() -> Color { Color::from_packed_value(4292664540) }

    pub fn GhostWhite() -> Color { Color::from_packed_value(4294965496) }

    pub fn Gold() -> Color { Color::from_packed_value(4278245375) }

    pub fn Goldenrod() -> Color { Color::from_packed_value(4280329690) }

    pub fn Gray() -> Color { Color::from_packed_value(4286611584) }

    pub fn Green() -> Color { Color::from_packed_value(4278222848) }

    pub fn GreenYellow() -> Color { Color::from_packed_value(4281335725) }

    pub fn Honeydew() -> Color { Color::from_packed_value(4293984240) }

    pub fn HotPink() -> Color { Color::from_packed_value(4290013695) }

    pub fn IndianRed() -> Color { Color::from_packed_value(4284243149) }

    pub fn Indigo() -> Color { Color::from_packed_value(4286709835) }

    pub fn Ivory() -> Color { Color::from_packed_value(4293984255) }

    pub fn Khaki() -> Color { Color::from_packed_value(4287424240) }

    pub fn Lavender() -> Color { Color::from_packed_value(4294633190) }

    pub fn LavenderBlush() -> Color { Color::from_packed_value(4294308095) }

    pub fn LawnGreen() -> Color { Color::from_packed_value(4278254716) }

    pub fn LemonChiffon() -> Color { Color::from_packed_value(4291689215) }

    pub fn LightBlue() -> Color { Color::from_packed_value(4293318829) }

    pub fn LightCoral() -> Color { Color::from_packed_value(4286611696) }

    pub fn LightCyan() -> Color { Color::from_packed_value(4294967264) }

    pub fn LightGoldenrodYellow() -> Color { Color::from_packed_value(4292016890) }

    pub fn LightGreen() -> Color { Color::from_packed_value(4287688336) }

    pub fn LightGray() -> Color { Color::from_packed_value(4292072403) }

    pub fn LightPink() -> Color { Color::from_packed_value(4290885375) }

    pub fn LightSalmon() -> Color { Color::from_packed_value(4286226687) }

    pub fn LightSeaGreen() -> Color { Color::from_packed_value(4289376800) }

    pub fn LightSkyBlue() -> Color { Color::from_packed_value(4294626951) }

    pub fn LightSlateGray() -> Color { Color::from_packed_value(4288252023) }

    pub fn LightSteelBlue() -> Color { Color::from_packed_value(4292789424) }

    pub fn LightYellow() -> Color { Color::from_packed_value(4292935679) }

    pub fn Lime() -> Color { Color::from_packed_value(4278255360) }

    pub fn LimeGreen() -> Color { Color::from_packed_value(4281519410) }

    pub fn Linen() -> Color { Color::from_packed_value(4293325050) }

    pub fn Magenta() -> Color { Color::from_packed_value(4294902015) }

    pub fn Maroon() -> Color { Color::from_packed_value(4278190208) }

    pub fn MediumAquamarine() -> Color { Color::from_packed_value(4289383782) }

    pub fn MediumBlue() -> Color { Color::from_packed_value(4291624960) }

    pub fn MediumOrchid() -> Color { Color::from_packed_value(4292040122) }

    pub fn MediumPurple() -> Color { Color::from_packed_value(4292571283) }

    pub fn MediumSeaGreen() -> Color { Color::from_packed_value(4285641532) }

    pub fn MediumSlateBlue() -> Color { Color::from_packed_value(4293814395) }

    pub fn MediumSpringGreen() -> Color { Color::from_packed_value(4288346624) }

    pub fn MediumTurquoise() -> Color { Color::from_packed_value(4291613000) }

    pub fn MediumVioletRed() -> Color { Color::from_packed_value(4286911943) }

    pub fn MidnightBlue() -> Color { Color::from_packed_value(4285536537) }

    pub fn MintCream() -> Color { Color::from_packed_value(4294639605) }

    pub fn MistyRose() -> Color { Color::from_packed_value(4292994303) }

    pub fn Moccasin() -> Color { Color::from_packed_value(4290110719) }

    pub fn NavajoWhite() -> Color { Color::from_packed_value(4289584895) }

    pub fn Navy() -> Color { Color::from_packed_value(4286578688) }

    pub fn OldLace() -> Color { Color::from_packed_value(4293326333) }

    pub fn Olive() -> Color { Color::from_packed_value(4278222976) }

    pub fn OliveDrab() -> Color { Color::from_packed_value(4280520299) }

    pub fn Orange() -> Color { Color::from_packed_value(4278232575) }

    pub fn OrangeRed() -> Color { Color::from_packed_value(4278207999) }

    pub fn Orchid() -> Color { Color::from_packed_value(4292243674) }

    pub fn PaleGoldenrod() -> Color { Color::from_packed_value(4289390830) }

    pub fn PaleGreen() -> Color { Color::from_packed_value(4288215960) }

    pub fn PaleTurquoise() -> Color { Color::from_packed_value(4293848751) }

    pub fn PaleVioletRed() -> Color { Color::from_packed_value(4287852763) }

    pub fn PapayaWhip() -> Color { Color::from_packed_value(4292210687) }

    pub fn PeachPuff() -> Color { Color::from_packed_value(4290370303) }

    pub fn Peru() -> Color { Color::from_packed_value(4282353101) }

    pub fn Pink() -> Color { Color::from_packed_value(4291543295) }

    pub fn Plum() -> Color { Color::from_packed_value(4292714717) }

    pub fn PowderBlue() -> Color { Color::from_packed_value(4293320880) }

    pub fn Purple() -> Color { Color::from_packed_value(4286578816) }

    pub fn Red() -> Color { Color::from_packed_value(4278190335) }

    pub fn RosyBrown() -> Color { Color::from_packed_value(4287598524) }

    pub fn RoyalBlue() -> Color { Color::from_packed_value(4292962625) }

    pub fn SaddleBrown() -> Color { Color::from_packed_value(4279453067) }

    pub fn Salmon() -> Color { Color::from_packed_value(4285694202) }

    pub fn SandyBrown() -> Color { Color::from_packed_value(4284523764) }

    pub fn SeaGreen() -> Color { Color::from_packed_value(4283927342) }

    pub fn SeaShell() -> Color { Color::from_packed_value(4293850623) }

    pub fn Sienna() -> Color { Color::from_packed_value(4281160352) }

    pub fn Silver() -> Color { Color::from_packed_value(4290822336) }

    pub fn SkyBlue() -> Color { Color::from_packed_value(4293643911) }

    pub fn SlateBlue() -> Color { Color::from_packed_value(4291648106) }

    pub fn SlateGray() -> Color { Color::from_packed_value(4287660144) }

    pub fn Snow() -> Color { Color::from_packed_value(4294638335) }

    pub fn SpringGreen() -> Color { Color::from_packed_value(4286578432) }

    pub fn SteelBlue() -> Color { Color::from_packed_value(4290019910) }

    pub fn Tan() -> Color { Color::from_packed_value(4287411410) }

    pub fn Teal() -> Color { Color::from_packed_value(4286611456) }

    pub fn Thistle() -> Color { Color::from_packed_value(4292394968) }

    pub fn Tomato() -> Color { Color::from_packed_value(4282868735) }

    pub fn Turquoise() -> Color { Color::from_packed_value(4291878976) }

    pub fn Violet() -> Color { Color::from_packed_value(4293821166) }

    pub fn Wheat() -> Color { Color::from_packed_value(4289978101) }

    pub fn White() -> Color { Color::from_packed_value(0xFFFFFFFF) }

    pub fn WhiteSmoke() -> Color { Color::from_packed_value(4294309365) }

    pub fn Yellow() -> Color { Color::from_packed_value(4278255615) }

    pub fn YellowGreen() -> Color { Color::from_packed_value(4281519514) }
}

impl IPackedVector for Color {
    fn to_vector4(&self) -> Vector4 {
        todo!()
    }

    fn from_vector4(vector4: &Vector4) {
        todo!()
    }
}




