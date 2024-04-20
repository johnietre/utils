use std::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};
use std::thread;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    n_threads: usize,
    tx: RwLock<Option<Sender<Job>>>,
    done: AAV<Barrier>,
}

impl ThreadPool {
    pub const fn new(n_threads: NonZeroUsize) -> Self {
        Self {
            n_threads,
            tx: RwLock::new(None),
            done: AAV::empty(),
        }
    }

    pub fn run(&self) {
        let (tx, rx) = channel();
        *self.tx.write().unwrap() = Some(tx);
        let pair = Arc::new((Mutex::new(rx), Condvar::new()));
        for _ in 0..self.n_threads {
            let pair = Arc::clone(&pair);
            thread::spawn(move || {
                let (lock, cv) = &*pair;
                let mut rx = lock.lock().unwrap();
                loop {
                    let job = match rx.try_recv() {
                        Ok(job) => job,
                        Err(TryRecvError::Empty) => {
                            rx = cv.wait(rx).unwrap();
                            continue;
                        }
                        Err(TryRecvError::Disconnected) => break,
                    };
                    (job)();
                }
            });
        }
    }

    pub fn submit_job(&self, f: Job) -> Result<(), Job> {
        if let Some(tx) = self.tx.lock().unwrap().as_ref() {
            tx.send(f).map_err(|e| e.0)
        } else {
            Err(f)
        }
    }

    /// Shuts down the thread pool worker receivers, returning false if the call didn't shutdown
    /// the pool.
    pub fn shutdown(&self) -> bool {
        self.tx.lock().unwrap().take().is_some()
    }

    pub fn wait(&self) {
        //
    }
}

struct Wg(Arc<(Mutex<usize>, Condvar)>);

impl Wg {
    pub const fn new() -> Self {
        Self(Arc::new((Mutex::new(1), Condvar::new())))
    }

    pub fn wait(&self) {
        let mut count = self.0 .0.lock().unwrap();
        *count -= 1;
        while *count != 0 {
            count = self.0 .1.wait(count).unwrap();
        }
    }
}

impl Clone for Wg {
    fn clone(&self) -> Self {
        *self.0 .0.lock().unwrap() += 1;
        Self(Arc::clone(&self.0))
    }
}

impl Drop for Wg {
    fn drop(&mut self) {
        let mut count = self.0 .0.lock().unwrap();
        *count -= 1;
        if *count == 0 {
            self.0 .1.notify_all();
        }
    }
}
