use std::thread;
use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

/// ThreadPool creates and cordinates the workers
/// it also handles sending the jobs to the workers
pub struct ThreadPool {
   workers : Vec<Worker>,
   sender : mpsc::SyncSender<Job>
}


impl ThreadPool {
   /// Create a new ThreadPool
   /// 
   /// The size is the number of threads in the pool,
   /// it also specifies the size of the channel that
   /// the workers listen to.
   /// 
   /// # Panics
   /// 
   /// The `new` function will panic if the size is zero 
   pub fn new(size: usize) -> ThreadPool {
      let mut workers = Vec::with_capacity(size);
      let (sender, receiver) = mpsc::sync_channel(size);
      let receiver = Arc::new(Mutex::new(receiver));

      for id in 0..size {
         workers.push(Worker::new(id, Arc::clone(&receiver)));
      }

      ThreadPool {workers, sender}
   }

   /// Send the desired job (closure) to the workers to execute
   /// when the sending channel is full the thread calling this function
   /// will be blocked waiting for a new spot to clear out
   pub fn execute<F>(&self, f: F)
   where
      F: FnOnce() + Send + 'static
   {
      let job = Box::new(f);
      self.sender.send(job).unwrap();
   }
}

/// The worker contains a thread that will wait on the sender
/// to provide a job to execute
pub struct Worker {
   id: usize,
   thread: thread::JoinHandle<()>
}

impl Worker {
   /// Create a new worker with an id, and a receiver to listen for new jobs
   /// which are closures to be called
   fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
      let thread = thread::spawn(move || loop {
         // Get the job and execute it
         let job = receiver.lock().unwrap().recv().unwrap();
         job();
      });

      Worker {id, thread}
   }
}