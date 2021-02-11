use std::thread;
use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

/// The worker contains a thread that will wait on the sender
/// to provide a job to execute
pub struct Worker {
   id: usize,
   thread: thread::JoinHandle<()>
}

impl Worker {
   /// Create a new worker with an id, and a receiver to listen for new jobs
   fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
      let thread = thread::spawn(move || loop {
         let job = receiver.lock().unwrap().recv().unwrap();

         job();
      });

      Worker {id, thread}
   }
}