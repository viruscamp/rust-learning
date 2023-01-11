use std::iter::FromIterator;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::{Instant, Duration};
use std::vec::Vec;

static MAX: i32 = 100;

fn mpmc_test<W, MW>(make_worker: MW)
where
    W: FnOnce() + Send + 'static,
    MW: Fn(i32, Arc<Mutex<Vec<i32>>>) -> W
{
    let queue = Arc::new(Mutex::new(Vec::from_iter(0..MAX)));
    let begin = Instant::now();
    let mut threads = vec![];
    for id in 0..4 {
        threads.push(thread::spawn(make_worker(id, queue.clone())));
    }
    threads.into_iter().all(|t| t.join().is_ok());
    println!("used: {}", Instant::now().duration_since(begin).as_micros());
}

#[test]
fn mpmc_test1() {
    mpmc_test(|id, queue| move || {
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

    mpmc_test(|id, queue| move || {
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