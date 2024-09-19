
#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::sync::Mutex;
    use std::thread::{self, Thread};
    use std::time::Duration;

    fn start_deadlock_check() {
        // Create a background thread which checks for deadlocks every 10s
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(10));
                let deadlocks = parking_lot::deadlock::check_deadlock();
                if deadlocks.is_empty() {
                    continue;
                }

                println!("{} deadlocks detected", deadlocks.len());
                for (i, threads) in deadlocks.iter().enumerate() {
                    println!("Deadlock #{}", i);
                    for t in threads {
                        println!("Thread Id {:#?}", t.thread_id());
                        println!("{:#?}", t.backtrace());
                    }
                }
                std::process::exit(1);
            }
        });
    }

    #[test]
    fn dead_lock_1() {
        use parking_lot::Mutex;

        start_deadlock_check();
        let vec_mutex = Mutex::new(vec![1,2,3]);
        // 会导致临时变量 MutexGuard 锁的有效期包括 while 循环体导致死锁
        while let Some(num) = vec_mutex.lock().pop() {
            if num == 1 {
                vec_mutex.lock().push(4);
            }
            println!("got {}", num);
            // drop(vec_mutex.lock()); 发生在每次循环结束后, 循环体外
        }
        // 临时变量不是在整个 while 结尾 drop 的, 否则只有 3, 2 都出不来
    }

    #[test]
    fn dead_lock_2() {
        let vec_mutex = Mutex::new(vec![1,2,3]);
        // 模拟 while 展开
        // 会导致临时变量 MutexGuard 锁的有效期包括循环体导致死锁
        loop {
            if let Some(num) = vec_mutex.lock().unwrap().pop() {
                if num == 1 {
                    vec_mutex.lock().unwrap().push(4);
                }
                println!("got {}", num);
            } else {
                break;
            }
            // drop(vec_mutex.lock()); 发生在每次循环结束后, 循环体外
        }
    }

    #[test]
    fn no_dead_lock() {
        let vec_mutex = Mutex::new(vec![1,2,3]);
        while let Some(num) = {
            let n = vec_mutex.lock().unwrap().pop();
            // 前一个 ; 分号是临时变量所在语句的结尾, 临时变量在此处 drop
            n
        } {
            if num == 1 {
                vec_mutex.lock().unwrap().push(4);
            }
            println!("got {}", num);
        }
    }
}
