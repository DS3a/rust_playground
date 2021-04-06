struct Object {
    width: u32,
    height: u32,
}

impl Object {
// Instance methods
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn make_longer(&mut self, factor: u32) {
        self.height *= factor;
    }
}

impl Object {
// Class, and static methods
    fn init(width: u32, height: u32) -> Object {
        return Object {width: width, height: height};
    }
}

fn area(obj: &Object) -> u32 {
    return obj.height * obj.width;
}

fn make_longer(obj: &mut Object) -> Object {
    obj.height *= 2;
    let a = Object{
        width: obj.width,
        height: obj.height,
    };
    return a;
}

fn main() {

    let mut square = Object{width: 5, height: 5};
    println!("Side length {} , The area is {}", square.width, area(&square));
    let mut a = make_longer(&mut square);
    let _b = make_longer(&mut a);
    println!("The rectange formed has a length of {}", square.height);
    print!("The rectange formed has a length of {}", a.height);
    println!("The area using methods is : {}", a.area());
    println!("The area using methods is : {}", a.area());

    a.make_longer(3);

    let c = Object::init(25, 5);
    println!("The area using methods is : {}", c.area());
}
