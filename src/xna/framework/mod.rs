use crate::xna::framework::graphics::{IPackedVector, PackUtils};
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

impl IPackedVector for Color {
    fn to_vector4(&self) -> Vector4 {
        todo!()
    }

    fn set_vector4(&self, vector4: &Vector4) {
        todo!()
    }
}

impl Color {
    pub fn from_packed_value(packed_value: u32) -> Color {
        Color { packed_value }
    }

    pub fn from_rgb(r: i32, g: i32, b: i32) -> Color {
        let r1;
        let g1;
        let b1;

        if ((r | g  | b) & -256) != 0 {
            r1 = Self::clamp_to_byte64(r as u64);
            g1 = Self::clamp_to_byte64(g as u64);
            b1 = Self::clamp_to_byte64(b as u64);
        } else {
            r1 = r;
            g1 = g;
            b1 = b;
        }

        let g2 = g1 << 8;
        let b2 = b1 << 16;

        let packed_value = (r1 | g2 | b2 | -16777216) as u32;
        Self::from_packed_value(packed_value)
    }

    pub fn from_rgba(r: i32, g: i32, b: i32, a: i32) -> Color {
        let r1;
        let g1;
        let b1;
        let a1;

        if ((r | g  | b | a) & -256) != 0 {
            r1 = Self::clamp_to_byte64(r as u64);
            g1 = Self::clamp_to_byte64(g as u64);
            b1 = Self::clamp_to_byte64(b as u64);
            a1 = Self::clamp_to_byte64(a as u64);
        } else {
            r1 = r;
            g1 = g;
            b1 = b;
            a1 = a;
        }

        let g2 = g1 << 8;
        let b2 = b1 << 16;
        let a2 = a1 << 24;

        let packed_value = (r1 | g2 | b2 | a2) as u32;
        Self::from_packed_value(packed_value)
    }

    pub fn from_float_rgb(r: f32, g: f32, b: f32) -> Color {
        let packed_value = Self::pack_helper(r, g, b, 1.0);
        Self::from_packed_value(packed_value)
    }

    pub fn from_float_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        let packed_value = Self::pack_helper(r, g, b, a);
        Self::from_packed_value(packed_value)
    }

    pub fn from_vector3(vector3: Vector3) -> Color {
        let packed_value = Self::pack_helper(vector3.x, vector3.y, vector3.z, 1.0);
        Self::from_packed_value(packed_value)
    }

    pub fn from_vector4(vector4: Vector4) -> Color {
        let packed_value = Self::pack_helper(vector4.x, vector4.y, vector4.z, vector4.w);
        Self::from_packed_value(packed_value)
    }

    pub fn from_nom_premultiplied(vector: Vector4) -> Color {
        let packed_value = Self::pack_helper(vector.x * vector.w, vector.y * vector.w, vector.z * vector.w, vector.w);
        Self::from_packed_value(packed_value)
    }

    pub fn from_nom_premultiplied_rgba(r: i32, g: i32, b: i32, a: i32) -> Color {
        let r1 = Self::clamp_to_byte64((r * a) as u64 / u8::MAX  as u64);
        let g1 = Self::clamp_to_byte64((g * a) as u64 / u8::MAX as u64);
        let b1 = Self::clamp_to_byte64((b * a) as u64 / u8::MAX as u64);
        let a1 = Self::clamp_to_byte64(a as u64);

        let g2 = g1 << 8;
        let b2 = b1 << 16;
        let a2 = a1 << 24;

        let packed_value = (r1 | g2 | b2 | a2) as u32;
        Self::from_packed_value(packed_value)
    }

    pub fn r(&self)
             -> u8 { self.packed_value as u8 }

    pub fn g(&self)
             -> u8 { (self.packed_value >> 8) as u8 }

    pub fn b(&self)
             -> u8 { (self.packed_value >> 16) as u8 }

    pub fn a(&self)
             -> u8 { (self.packed_value >> 24) as u8 }

    pub fn clamp_to_byte64(value: u64) -> i32 {
        if value < 0{
            return 0;
        }

        if value > u8::MAX as u64{
            return u8::MAX as i32;
        }

        value as i32
    }

    pub fn pack_helper(vector_x: f32, vector_y: f32, vector_z: f32, vector_w: f32) -> u32 {
        PackUtils::pack_unorm(u8::MAX as f32, vector_x)
            | PackUtils::pack_unorm(u8::MAX as f32, vector_y) << 8
            | PackUtils::pack_unorm(u8::MAX as f32, vector_z) << 16
            | PackUtils::pack_unorm(u8::MAX as f32, vector_w) << 24
    }


    //
    // COLORS
    //

    pub fn transparent() -> Color { Color::from_packed_value(0) }

    pub fn alice_blue() -> Color { Color::from_packed_value(4294965488) }

    pub fn antique_white() -> Color { Color::from_packed_value(4292340730) }

    pub fn aqua() -> Color { Color::from_packed_value(4294967040) }

    pub fn aquamarine() -> Color { Color::from_packed_value(4292149119) }

    pub fn azure() -> Color { Color::from_packed_value(4294967280) }

    pub fn beige() -> Color { Color::from_packed_value(4292670965) }

    pub fn bisque() -> Color { Color::from_packed_value(4291093759) }

    pub fn black() -> Color { Color::from_packed_value(4278190080) }

    pub fn blanched_almond() -> Color { Color::from_packed_value(4291685375) }

    pub fn blue() -> Color { Color::from_packed_value(4294901760) }

    pub fn blue_violet() -> Color { Color::from_packed_value(4293012362) }

    pub fn brown() -> Color { Color::from_packed_value(4280953509) }

    pub fn burly_wood() -> Color { Color::from_packed_value(4287084766) }

    pub fn cadet_blue() -> Color { Color::from_packed_value(4288716383) }

    pub fn chartreuse() -> Color { Color::from_packed_value(4278255487) }

    pub fn chocolate() -> Color { Color::from_packed_value(4280183250) }

    pub fn coral() -> Color { Color::from_packed_value(4283465727) }

    pub fn cornflower_blue() -> Color { Color::from_packed_value(4293760356) }

    pub fn cornsilk() -> Color { Color::from_packed_value(4292671743) }

    pub fn crimson() -> Color { Color::from_packed_value(4282127580) }

    pub fn cyan() -> Color { Color::from_packed_value(4294967040) }

    pub fn dark_blue() -> Color { Color::from_packed_value(4287299584) }

    pub fn dark_cyan() -> Color { Color::from_packed_value(4287335168) }

    pub fn dark_goldenrod() -> Color { Color::from_packed_value(4278945464) }

    pub fn dark_gray() -> Color { Color::from_packed_value(4289309097) }

    pub fn dark_green() -> Color { Color::from_packed_value(4278215680) }

    pub fn dark_khaki() -> Color { Color::from_packed_value(4285249469) }

    pub fn dark_magenta() -> Color { Color::from_packed_value(4287299723) }

    pub fn dark_olive_green() -> Color { Color::from_packed_value(4281297749) }

    pub fn dark_orange() -> Color { Color::from_packed_value(4278226175) }

    pub fn dark_orchid() -> Color { Color::from_packed_value(4291572377) }

    pub fn dark_red() -> Color { Color::from_packed_value(4278190219) }

    pub fn dark_salmon() -> Color { Color::from_packed_value(4286224105) }

    pub fn dark_sea_green() -> Color { Color::from_packed_value(4287347855) }

    pub fn dark_slate_blue() -> Color { Color::from_packed_value(4287315272) }

    pub fn dark_slate_gray() -> Color { Color::from_packed_value(4283387695) }

    pub fn dark_turquoise() -> Color { Color::from_packed_value(4291939840) }

    pub fn dark_violet() -> Color { Color::from_packed_value(4292018324) }

    pub fn deep_pink() -> Color { Color::from_packed_value(4287829247) }

    pub fn deep_sky_blue() -> Color { Color::from_packed_value(4294950656) }

    pub fn dim_gray() -> Color { Color::from_packed_value(4285098345) }

    pub fn dodger_blue() -> Color { Color::from_packed_value(4294938654) }

    pub fn firebrick() -> Color { Color::from_packed_value(4280427186) }

    pub fn floral_white() -> Color { Color::from_packed_value(4293982975) }

    pub fn forest_green() -> Color { Color::from_packed_value(4280453922) }

    pub fn fuchsia() -> Color { Color::from_packed_value(4294902015) }

    pub fn gainsboro() -> Color { Color::from_packed_value(4292664540) }

    pub fn ghost_white() -> Color { Color::from_packed_value(4294965496) }

    pub fn gold() -> Color { Color::from_packed_value(4278245375) }

    pub fn goldenrod() -> Color { Color::from_packed_value(4280329690) }

    pub fn gray() -> Color { Color::from_packed_value(4286611584) }

    pub fn green() -> Color { Color::from_packed_value(4278222848) }

    pub fn green_yellow() -> Color { Color::from_packed_value(4281335725) }

    pub fn honeydew() -> Color { Color::from_packed_value(4293984240) }

    pub fn hot_pink() -> Color { Color::from_packed_value(4290013695) }

    pub fn indian_red() -> Color { Color::from_packed_value(4284243149) }

    pub fn indigo() -> Color { Color::from_packed_value(4286709835) }

    pub fn ivory() -> Color { Color::from_packed_value(4293984255) }

    pub fn khaki() -> Color { Color::from_packed_value(4287424240) }

    pub fn lavender() -> Color { Color::from_packed_value(4294633190) }

    pub fn lavender_blush() -> Color { Color::from_packed_value(4294308095) }

    pub fn lawn_green() -> Color { Color::from_packed_value(4278254716) }

    pub fn lemon_chiffon() -> Color { Color::from_packed_value(4291689215) }

    pub fn light_blue() -> Color { Color::from_packed_value(4293318829) }

    pub fn light_coral() -> Color { Color::from_packed_value(4286611696) }

    pub fn light_cyan() -> Color { Color::from_packed_value(4294967264) }

    pub fn light_goldenrod_yellow() -> Color { Color::from_packed_value(4292016890) }

    pub fn light_green() -> Color { Color::from_packed_value(4287688336) }

    pub fn light_gray() -> Color { Color::from_packed_value(4292072403) }

    pub fn light_pink() -> Color { Color::from_packed_value(4290885375) }

    pub fn light_salmon() -> Color { Color::from_packed_value(4286226687) }

    pub fn light_sea_green() -> Color { Color::from_packed_value(4289376800) }

    pub fn light_sky_blue() -> Color { Color::from_packed_value(4294626951) }

    pub fn light_slate_gray() -> Color { Color::from_packed_value(4288252023) }

    pub fn light_steel_blue() -> Color { Color::from_packed_value(4292789424) }

    pub fn light_yellow() -> Color { Color::from_packed_value(4292935679) }

    pub fn lime() -> Color { Color::from_packed_value(4278255360) }

    pub fn lime_green() -> Color { Color::from_packed_value(4281519410) }

    pub fn linen() -> Color { Color::from_packed_value(4293325050) }

    pub fn magenta() -> Color { Color::from_packed_value(4294902015) }

    pub fn maroon() -> Color { Color::from_packed_value(4278190208) }

    pub fn medium_aquamarine() -> Color { Color::from_packed_value(4289383782) }

    pub fn medium_blue() -> Color { Color::from_packed_value(4291624960) }

    pub fn medium_orchid() -> Color { Color::from_packed_value(4292040122) }

    pub fn medium_purple() -> Color { Color::from_packed_value(4292571283) }

    pub fn medium_sea_green() -> Color { Color::from_packed_value(4285641532) }

    pub fn medium_slate_blue() -> Color { Color::from_packed_value(4293814395) }

    pub fn medium_spring_green() -> Color { Color::from_packed_value(4288346624) }

    pub fn medium_turquoise() -> Color { Color::from_packed_value(4291613000) }

    pub fn medium_violet_red() -> Color { Color::from_packed_value(4286911943) }

    pub fn midnight_blue() -> Color { Color::from_packed_value(4285536537) }

    pub fn mint_cream() -> Color { Color::from_packed_value(4294639605) }

    pub fn misty_rose() -> Color { Color::from_packed_value(4292994303) }

    pub fn moccasin() -> Color { Color::from_packed_value(4290110719) }

    pub fn navajo_white() -> Color { Color::from_packed_value(4289584895) }

    pub fn navy() -> Color { Color::from_packed_value(4286578688) }

    pub fn old_lace() -> Color { Color::from_packed_value(4293326333) }

    pub fn olive() -> Color { Color::from_packed_value(4278222976) }

    pub fn olive_drab() -> Color { Color::from_packed_value(4280520299) }

    pub fn orange() -> Color { Color::from_packed_value(4278232575) }

    pub fn orange_red() -> Color { Color::from_packed_value(4278207999) }

    pub fn orchid() -> Color { Color::from_packed_value(4292243674) }

    pub fn pale_goldenrod() -> Color { Color::from_packed_value(4289390830) }

    pub fn pale_green() -> Color { Color::from_packed_value(4288215960) }

    pub fn pale_turquoise() -> Color { Color::from_packed_value(4293848751) }

    pub fn pale_violet_red() -> Color { Color::from_packed_value(4287852763) }

    pub fn papaya_whip() -> Color { Color::from_packed_value(4292210687) }

    pub fn peach_puff() -> Color { Color::from_packed_value(4290370303) }

    pub fn peru() -> Color { Color::from_packed_value(4282353101) }

    pub fn pink() -> Color { Color::from_packed_value(4291543295) }

    pub fn plum() -> Color { Color::from_packed_value(4292714717) }

    pub fn powder_blue() -> Color { Color::from_packed_value(4293320880) }

    pub fn purple() -> Color { Color::from_packed_value(4286578816) }

    pub fn red() -> Color { Color::from_packed_value(4278190335) }

    pub fn rosy_brown() -> Color { Color::from_packed_value(4287598524) }

    pub fn royal_blue() -> Color { Color::from_packed_value(4292962625) }

    pub fn saddle_brown() -> Color { Color::from_packed_value(4279453067) }

    pub fn salmon() -> Color { Color::from_packed_value(4285694202) }

    pub fn sandy_brown() -> Color { Color::from_packed_value(4284523764) }

    pub fn sea_green() -> Color { Color::from_packed_value(4283927342) }

    pub fn sea_shell() -> Color { Color::from_packed_value(4293850623) }

    pub fn sienna() -> Color { Color::from_packed_value(4281160352) }

    pub fn silver() -> Color { Color::from_packed_value(4290822336) }

    pub fn sky_blue() -> Color { Color::from_packed_value(4293643911) }

    pub fn slate_blue() -> Color { Color::from_packed_value(4291648106) }

    pub fn slate_gray() -> Color { Color::from_packed_value(4287660144) }

    pub fn snow() -> Color { Color::from_packed_value(4294638335) }

    pub fn spring_green() -> Color { Color::from_packed_value(4286578432) }

    pub fn steel_blue() -> Color { Color::from_packed_value(4290019910) }

    pub fn tan() -> Color { Color::from_packed_value(4287411410) }

    pub fn teal() -> Color { Color::from_packed_value(4286611456) }

    pub fn thistle() -> Color { Color::from_packed_value(4292394968) }

    pub fn tomato() -> Color { Color::from_packed_value(4282868735) }

    pub fn turquoise() -> Color { Color::from_packed_value(4291878976) }

    pub fn violet() -> Color { Color::from_packed_value(4293821166) }

    pub fn wheat() -> Color { Color::from_packed_value(4289978101) }

    pub fn white() -> Color { Color::from_packed_value(0xFFFFFFFF) }

    pub fn white_smoke() -> Color { Color::from_packed_value(4294309365) }

    pub fn yellow() -> Color { Color::from_packed_value(4278255615) }

    pub fn yellow_green() -> Color { Color::from_packed_value(4281519514) }
}
