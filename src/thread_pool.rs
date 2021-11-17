use log::debug;
use std::fmt;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            debug!("Cannot create pool with size {}", size);
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            debug!("Creating worker {}", id);
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // TODO: gracefully handle error instead of unwrap()
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        debug!("Sending terminate message to all workers.");

        // Using two loops to avoid deadlock

        debug!("Sending termination message to workers");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        debug!("Shutting down all workers.");

        for worker in &mut self.workers {
            debug!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                // TODO: gracefully handle error instead of unwrap()
                thread.join().unwrap();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid pool size (must be greater than 0)")
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // TODO: gracefully handle error instead of unwrap()
            let message = receiver
                .lock()
                .expect("Cannot acquire lock, mutex might be in poisoned state")
                .recv() // `recv` blocks until a job becomes available
                .unwrap();

            match message {
                Message::NewJob(job) => {
                    debug!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    debug!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    mod thread_pool {
        use super::super::*;

        #[test]
        fn new_should_return_error_for_size_equals_0() {
            assert!(ThreadPool::new(0).is_err());
        }

        #[test]
        fn new_should_return_a_thread_pool_with_workers_len_equals_size() {
            const SIZE: usize = 4;

            assert_eq!(ThreadPool::new(SIZE).unwrap().workers.len(), SIZE);
        }
    }
}
