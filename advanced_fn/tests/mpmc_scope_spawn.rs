use std::iter::FromIterator;
use std::sync::Mutex;
use std::thread;
use std::time::{Instant, Duration};
use std::vec::Vec;

static MAX: i32 = 100;

fn mpmc_test_scope_worker<W>(worker: W)
where
    for<'a> W: Fn(i32, &'a Mutex<Vec<i32>>) + Sync
{
    let queue = Mutex::new(Vec::from_iter(0..MAX));
    let begin = Instant::now();
    thread::scope(|s| {
        for id in 0..4 {
            let queue = &queue;
            let worker = &worker;
            s.spawn(move || worker(id, queue));
        }
    });
    println!("used: {}", Instant::now().duration_since(begin).as_micros());
}

#[test]
fn mpmc_test_scope_worker_1() {
    mpmc_test_scope_worker(|id, queue| {
        let mut count = 0;
        let mut sum = 0i64;
        while let Some(num) = queue.lock().unwrap().pop() {
            //println!("id:{} got {}", id, num);
            sum += num as i64;
            count += 1;
            thread::sleep(Duration::new(0, 10));
        }
        println!("thread:{id} done with count={count} sum={sum}");
    });

    mpmc_test_scope_worker(|id, queue| {
        let mut count = 0;
        let mut sum = 0i64;
        while let Some(num) = { let num = queue.lock().unwrap().pop(); num } {
            //println!("id:{} got {}", id, num);
            sum += num as i64;
            count += 1;            
            thread::sleep(Duration::new(0, 10));
        }
        println!("thread:{id} done with count={count} sum={sum}");
    });
}
