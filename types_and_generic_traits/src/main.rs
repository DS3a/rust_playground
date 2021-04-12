use std::fmt;
mod generic_types;

trait MyTrait {
    fn sum(&self) -> Option<f64>;
}

#[derive(Debug)]
struct A {
    a: u8,
    b: f64,
}

impl Copy for A {}
impl Clone for A {
    fn clone(&self) -> A {
        *self
    }
}



impl MyTrait for A {
    fn sum(&self) -> Option<f64> {
        return Option::Some(self.a as f64 + self.b)
    }
}

fn print_sum(obj: &impl MyTrait) { // printsum will take in any datatype which implements our trait
    println!("The sum is {:?}", obj.sum());
}

impl fmt::Display for A {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self.sum());
    }
}

impl A {
    fn kill(self) {
        println!("Killing {}", self);
    }
}

fn main(){
    let a  = A{a: 1, b: 23.9};
    println!("{}", a);
    print_sum(&a);
    a.kill();
//    print_sum(&a); // You can't do this as the kill function is called, 
// it's not a default function, but you can see in the
// implementation that it kills the instance

    let a = vec![1, 3, 4, 22];
    generic_types::largest(&a);

    let a = vec!['a', 'b', 'v', 'c', 'd'];
    generic_types::largest(&a);

    let b = A{a: 1, b: 24 as f64};
    generic_types::time_to_run(|| {b.kill()});
    let c = generic_types::N{x: 1 as u8,y: 1 as u8};
}