fn main() {
/*
    let mut a: u8 = 2;
    let mut b = & a;
    let mut c = &mut b;
    c = &mut &a;
    let x: *mut u8 = &mut a;
    unsafe {*x += 1 };
    println!{"{:?}", x};
*/

    let mut x = 24;
    let y = &mut x;
    *y += 1; 

    let a : Option<i32> = Some(2);
    let b: Option<i32> = None;
    if let Option::Some(x) = b {
        println!("{:?}", x);
    } else {
        println!("None {{lmao}}");
    }
}
