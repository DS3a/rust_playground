fn main() {
    let mut a: u8 = 25;
    let b: u8 = 27;

/*    let ar = &a;
    let ar2 = &a;
    let ar3 = &a;*/
    let ar4 = &mut a;
    let ar5 = &mut a;

    let br = &b;

    println!("{}", *br==*ar5);
}
