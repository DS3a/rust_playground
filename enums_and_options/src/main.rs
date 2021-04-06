use std::fmt;
#[allow(dead_code)]

#[derive(Debug)]
enum Shape {
    SQUARE {side: f32},
    CIRCLE {radius: f32},
    RECTANGLE {length: f32, breadth: f32},
}

impl Shape {
    fn area(&self) -> f32 {
        match *self {
            Shape::CIRCLE {radius: r} => return 3.14*r*r,
            Shape::SQUARE {side: s} => return s*s,
            Shape::RECTANGLE {length: l, breadth: b} => return l*b,
        }
    }

    fn vertices(&self) -> Option<u8> {
        match *self {
            Shape::CIRCLE {radius : _r} => return Option::None,
            _ => return Option::Some(4),
        }

    }

    fn enlarge(self: &mut Shape, factor: f32) {
        match *self {
            Shape::CIRCLE {radius: ref mut r} => *r *= factor,
            Shape::SQUARE {side: ref mut s} => *s *= factor,
            Shape::RECTANGLE {length: ref mut l, breadth: ref mut b} => {
                *l *= factor;
                *b *= factor;
            },
        }
    }

}
/*
impl fmt::Display for Option<T> {
    fn fmt(o: Option<T>) {

    }
}
*/
fn main() {
    let mut square_side: f32 = 0.0;
    let mut square: Shape = Shape::SQUARE{side: 24.0};
    if let Shape::SQUARE {side: ss} = &square{
        println!("{}", ss);
        square_side = *ss;
    }
    println!("side length {}, area = {}", square_side, square.area());
    square.enlarge(0.5);
    println!("new area = {}", square.area());
    square.enlarge(2 as f32);
    println!("new area = {}", square.area());
//    println!("no vertices = {}", square.vertices());
}
