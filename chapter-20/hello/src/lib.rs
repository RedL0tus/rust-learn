use std::thread;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;

type Job = Box<FnBox + Send + 'static>;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        return (*self)();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct PoolCreationError;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Work {} got a job; executing.", id);
                job.call_box();
            }
        });
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
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            return Ok(ThreadPool {
                workers,
                sender,
            });
        } else {
            return Err(PoolCreationError);
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
