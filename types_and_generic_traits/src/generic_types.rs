use std::time::Instant;

pub fn largest<T: PartialOrd+Copy+std::fmt::Display>(list: &[T]) -> T {
    let mut largestt = list[0];
    for &item in list {
        if item > largestt {
            largestt = item;
        }
    }
    println!("The largest value is {}", largestt);
    return largestt;
}

pub struct N<T, Y> {
    pub x: T,
    pub y: Y,
}

pub fn time_to_run<T>(func: T)
where T: Fn() -> (){
    let time_start = Instant::now();
    func();
    let time_end = Instant::now();
    println!("The function took {:?} to run", time_end-time_start);
}