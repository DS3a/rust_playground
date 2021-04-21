use std::thread;

//Two types of threads
//      OS THREAD, offered by the operating system
//      The GREEN THREAD, an abstraction which sits on top of the OS thread 

fn main() {
    let mut c = vec![];

    for i in 0..10 {
        c.push(thread::spawn(move || {
            println!("Thread number {}", i);
        }));
    }

    for _j in 0..10 {
        println!("Main thread");
    }

    for thread in c {
        thread.join();
    }
}
