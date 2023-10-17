// threads1.rs
//
// This program spawns multipow much time they took to complete. The program should
// wait until all the spale threads that each run for at least 250ms, and
// each thread returns hwned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.



use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        /*
        创建了10个线程，每个线程睡眠250毫秒，然后打印自己的编号和耗时
        使用JoinHandle的join方法来等待每个线程结束，并获取它们的返回值
        */
       let result = handle.join().unwrap();
       results.push(result);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
    

}
