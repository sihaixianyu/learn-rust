use std::{
    sync::{Arc, Barrier, Mutex, Condvar, Once, mpsc},
    thread,
};

pub fn barrier_example() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn condvar_example() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move|| {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("started changed");
}

static mut VAL: usize = 0;
static INIT: Once = Once::new();

pub fn once_example() {
    let handle1 = thread::spawn(move || {
        INIT.call_once(|| {
            unsafe {
                VAL = 1;
            }
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| {
            unsafe {
                VAL = 2;
            }
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}

pub fn mpsc_exmaple() {
    // Create a message and return a tuple: (sender, receiver)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = 1;
        println!("send {}", val);
        tx.send(val).unwrap();
    });

    // println!("receive {}", rx.recv().unwrap());
    loop {
        if let Ok(val) = rx.try_recv() {
            println!("receive {}", val);
            break
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barrier_example() {
        barrier_example();
    }

    #[test]
    fn test_condvar_example() {
        condvar_example();
    }

    #[test]
    fn test_once_example() {
        once_example();
    }

    #[test]
    fn test_mpsc_example() {
        mpsc_exmaple();
    }
}
