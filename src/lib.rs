use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct Worker {
   id: usize,
   thread: thread::JoinHandle<()>
}

impl Worker {
   fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
      let thread = thread::spawn(move || loop {
         let job = receiver.lock().unwrap().recv().unwrap();

         job();
      });

      Worker {id, thread}
   }
}