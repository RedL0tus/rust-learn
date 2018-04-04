use std::thread;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

pub struct PoolCreationError;

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        return Worker {
            id,
            thread,
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Result
    ///
    /// The `new` function will return Ok(ThreadPool) or Err(PoolCreationError).

    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id));
            }
            return Ok(ThreadPool {
                workers
            });
        } else {
            return Err(PoolCreationError);
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
    }
}
